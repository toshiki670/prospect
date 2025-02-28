use axum::Router;
use database::DatabaseState;
use std::sync::Arc;

mod database;
mod router;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db_state = Arc::new(DatabaseState::new());
    let router = Router::new().with_state(db_state.clone());

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .manage(router::RouterAdapterState::new(router))
        .invoke_handler(tauri::generate_handler![router::axum_api])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
