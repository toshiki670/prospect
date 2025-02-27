use axum::Router;

mod database;
mod router;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let router = Router::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .manage(database::DatabaseState::new())
        .manage(router::RouterAdapterState::new(router))
        .invoke_handler(tauri::generate_handler![router::axum_api])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
