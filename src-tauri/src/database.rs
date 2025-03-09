use anyhow::Context as _;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::{path::BaseDirectory, State};
use tokio::sync::RwLock;

pub type DatabaseState = Arc<RwLock<OptionalDatabaseConnection>>;
pub type OptionalDatabaseConnection = Option<DatabaseConnection>;

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Database connection not established")]
    ConnectionNotEstablished,
}

pub fn initialize() -> DatabaseState {
    DatabaseState::new(RwLock::new(None))
}

#[allow(dead_code)] // TODO: delete after using
pub async fn establish_connection(state: DatabaseState, path: PathBuf) -> anyhow::Result<()> {
    let db_path = path.join("database.db");
    let db_url = format!("sqlite:{}", db_path.display());

    // 接続オプションの設定
    let mut opt = ConnectOptions::new(&db_url);
    opt.max_connections(20)
        .min_connections(5)
        .sqlx_logging(true);

    // データベース接続の作成
    let db = Database::connect(opt)
        .await
        .with_context(|| format!("Failed to connect to database: {}", db_url))?;

    state.write().await.replace(db);
    Ok(())
}

pub async fn has_established_connection(state: &DatabaseState) -> bool {
    let connection = state.read().await;
    connection.is_some()
}

pub async fn get_connection(state: &DatabaseState) -> Result<DatabaseConnection, DatabaseError> {
    if !has_established_connection(state).await {
        return Err(DatabaseError::ConnectionNotEstablished);
    }

    // Acquire a read lock on the RwLock. This allows multiple readers to access the data concurrently,
    // but blocks if there's a writer holding the lock.
    let locked_state = state.read().await;

    // Get a reference to the connection if it exists
    let connection_ref = locked_state.as_ref();

    if let Some(connection) = connection_ref {
        Ok(connection.clone())
    } else {
        unreachable!() // because checked at the beginning
    }
}
