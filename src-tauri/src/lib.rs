mod database;
mod router;

mod application;
mod domain;
mod infrastructure;
mod interface;

use axum::{
    Router,
    extract::{Path, Query, State},
    routing::{get, post},
};
use interface::import::import_tokyo_stock_exchange;
use ipc_if::sample::*;
use router::AxumResult;
use tauri::{Manager as _, path::BaseDirectory};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let db_state = database::initialize();

    let router = Router::new()
        .route("/sample/{id}", get(sample))
        .route(
            "/import/tokyo_stock_exchange",
            post(import_tokyo_stock_exchange),
        )
        .with_state(db_state.clone());

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .manage(db_state.clone())
        .manage(router::RouterAdapterState::new(router))
        .invoke_handler(tauri::generate_handler![router::axum_api])
        .setup(|app| {
            let path = app
                .path()
                .resolve("database.db", BaseDirectory::AppLocalData)?;

            tauri::async_runtime::spawn(async move {
                let db_state = db_state.clone();
                database::establish_connection(&db_state, path)
                    .await
                    .unwrap(); // TODO: handle error

                let pool = database::get_connection(&db_state).await.unwrap(); // TODO: handle error

                database::migrate_database(&pool).await.unwrap(); // TODO: handle error
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn sample(
    State(pool): State<database::DatabaseState>,
    Path(_id): Path<u32>,
    Query(_params): Query<SampleQuery>,
) -> AxumResult<axum::Json<Sample>> {
    let _pool = database::get_connection(&pool)
        .await?;

    Ok(axum::Json(Sample {
        id: 1,
        title: "title".to_string(),
        body: "body".to_string(),
    }))
}
