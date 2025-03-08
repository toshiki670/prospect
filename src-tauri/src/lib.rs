use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::get,
};
use database::DatabaseState;
use ipc_if::sample::*;
use std::sync::Arc;

mod database;
mod router;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db_state = Arc::new(DatabaseState::new());
    let router = Router::new()
        .route("/sample/:id", get(sample))
        .with_state(db_state.clone());

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .manage(db_state.clone())
        .manage(router::RouterAdapterState::new(router))
        .invoke_handler(tauri::generate_handler![router::axum_api])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn sample(
    State(_pool): State<Arc<DatabaseState>>,
    Path(_id): Path<u32>,
    Query(_params): Query<SampleQuery>,
) -> Result<Json<Sample>, String> {
    Ok(Json(Sample {
        id: 1,
        title: "title".to_string(),
        body: "body".to_string(),
    }))
}
