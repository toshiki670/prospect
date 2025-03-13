mod database;
mod infrastructure;
mod router;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};
use ipc_if::sample::*;
use tauri::{Manager as _, path::BaseDirectory};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let db_state = database::initialize();

    let router = Router::new()
        .route("/sample/{id}", get(sample))
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
) -> Result<Json<Sample>, (StatusCode, String)> {
    let _pool = database::get_connection(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(Sample {
        id: 1,
        title: "title".to_string(),
        body: "body".to_string(),
    }))
}
