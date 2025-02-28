use anyhow::Context as _;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{path::Path, sync::Arc};

pub struct DatabaseState(Option<DatabaseConnection>);

impl core::ops::Deref for DatabaseState {
    type Target = Option<DatabaseConnection>;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}

impl DatabaseState {
    pub fn new() -> Self {
        Self(None)
    }

    #[allow(dead_code)] // TODO: delete after using
    pub async fn initialize(
        &mut self,
        database_path: &str,
    ) -> anyhow::Result<Arc<DatabaseConnection>> {
        // アプリデータディレクトリを取得
        let app_dir = Path::new(database_path);
        std::fs::create_dir_all(&app_dir)
            .with_context(|| format!("Failed to create app directory: {}", app_dir.display()))?;

        let db_path = app_dir.join("database.db");
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

        Ok(Arc::new(db))
    }
}
