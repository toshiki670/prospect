// src-tauri/src/router.rs
use axum::{body::Body, http::Request, routing::Router};
use serde_json::{Value, json};
use std::sync::Arc;
use tower::ServiceExt as _;

pub struct RouterAdapterState(Arc<RouterAdapter>);

impl RouterAdapterState {
    pub fn new(router: Router) -> Self {
        Self(Arc::new(RouterAdapter::new(router)))
    }
}

pub struct RouterAdapter {
    router: Router,
}

impl RouterAdapter {
    pub fn new(router: Router) -> Self {
        Self { router }
    }

    pub async fn handle_command(
        &self,
        path: String,
        method: String,
        payload: Option<Value>,
    ) -> Result<Value, String> {
        // リクエストの構築
        let uri = path
            .parse::<axum::http::Uri>()
            .map_err(|e| format!("Invalid path: {}", e))?;
        let body = payload.unwrap_or(json!({}));
        let body_bytes = serde_json::to_vec(&body).map_err(|e| e.to_string())?;

        // axumルーターが処理できるリクエストを構築
        let method = method
            .parse::<axum::http::Method>()
            .map_err(|e| format!("Invalid method: {}", e))?;
        let request = Request::builder()
            .uri(uri)
            .method(method)
            .header("content-type", "application/json")
            .body(Body::from(body_bytes))
            .map_err(|e| e.to_string())?;

        // axumルーターでリクエストを処理
        let response = self
            .router
            .clone()
            .oneshot(request)
            .await
            .map_err(|e| format!("Router error: {}", e))?;

        // レスポンスの解析
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .map_err(|e| e.to_string())?;

        if status != axum::http::StatusCode::OK {
            return Err(format!("Error: status code {}", status));
        }

        let response_value: Value =
            serde_json::from_slice(&bytes).map_err(|e| format!("Response parsing error: {}", e))?;

        Ok(response_value)
    }
}

// Tauriコマンドとして登録
#[tauri::command]
pub async fn axum_api(
    path: String,
    method: String,
    payload: Option<Value>,
    adapter: tauri::State<'_, RouterAdapterState>,
) -> Result<Value, String> {
    adapter.0.handle_command(path, method, payload).await
}
