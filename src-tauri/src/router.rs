use std::{collections::HashMap, sync::Arc};

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

#[tauri::command]
pub async fn api_call(
    path: String,
    payload: Value,
    router: tauri::State<'_, Arc<Router>>,
) -> Result<Value, String> {
    router.handle(&path, payload)
}

type RouteHandler = Box<dyn Fn(Value) -> Result<Value, String> + Send + Sync>;

pub struct Router {
    routes: HashMap<String, RouteHandler>,
}

impl Router {
    #[allow(dead_code)] // TODO: delete after using
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    #[allow(dead_code)] // TODO: delete after using
    pub fn add<F, Req, Res>(&mut self, path: &str, handler: F)
    where
        F: Fn(Req) -> Result<Res, String> + Send + Sync + 'static,
        Req: DeserializeOwned,
        Res: Serialize,
    {
        let path = path.to_string();
        let boxed_handler = Box::new(move |payload: Value| -> Result<Value, String> {
            let req = serde_json::from_value(payload).map_err(|e| e.to_string())?;
            let res = handler(req)?;
            serde_json::to_value(res).map_err(|e| e.to_string())
        });

        self.routes.insert(path, boxed_handler);
    }

    pub fn handle(&self, path: &str, payload: Value) -> Result<Value, String> {
        match self.routes.get(path) {
            Some(handler) => handler(payload),
            None => Err(format!("Route not found: {}", path)),
        }
    }
}

// main.rs
// fn main() {
//     let mut router = Router::new();

//     // ユーザー関連のルート
//     router.add("users/get", |req: GetUserRequest| -> Result<User, String> {
//         // 実装...
//     });

//     router.add(
//         "users/create",
//         |req: CreateUserRequest| -> Result<User, String> {
//             // 実装...
//         },
//     );

//     // タスク関連のルート
//     router.add(
//         "tasks/list",
//         |_: EmptyRequest| -> Result<Vec<Task>, String> {
//             // 実装...
//         },
//     );

//     // ルーター自体をステートとして管理
//     let router = Arc::new(router);
//     let router_clone = router.clone();

//     #[tauri::command]
//     async fn api_call(
//         path: String,
//         payload: Value,
//         router: State<'_, Arc<Router>>,
//     ) -> Result<Value, String> {
//         router.handle(&path, payload)
//     }

//     tauri::Builder::default()
//         .manage(router_clone)
//         .invoke_handler(tauri::generate_handler![api_call])
//         .run(tauri::generate_context!())
//         .expect("error while running application");
// }
