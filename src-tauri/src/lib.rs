use axum::{
    extract::{Path, Query, State}, routing::get, Json, Router
};
use database::DatabaseState;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

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

#[derive(TS, Serialize, Deserialize, Debug)]
#[ts(export, export_to = "sample.ts")]
pub struct SampleQuery {
    pub query: Option<String>,
}

#[derive(TS, Serialize, Deserialize, Debug)]
#[ts(export, export_to = "sample.ts")]
pub struct SampleResponse {
    pub id: u32,
    pub title: String,
    pub body: String,
}

async fn sample(
    State(_pool): State<Arc<DatabaseState>>,
    Path(_id): Path<u32>,
    Query(_params): Query<SampleQuery>,
) -> Result<Json<SampleResponse>, String> {
    Ok(Json(SampleResponse {
        id: 1,
        title: "title".to_string(),
        body: "body".to_string(),
    }))
}
