use std::{path::PathBuf, sync::Arc};

use anyhow::Context as _;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use ipc_if::import::TokyoStockExchangeImportQuery;

use crate::{
    app_error::AppError,
    application::tokyo_stock_exchange::import_files::ImportFilesUseCase,
    database,
    infrastructure::{
        file_system::repositories::tokyo_stock_exchange_repository::TokyoStockExchangeFileRepositoryImpl,
        persistence::repositories::tokyo_stock_exchange_repository::TokyoStockExchangeRepositoryImpl,
    },
};

pub async fn import_tokyo_stock_exchange(
    State(pool): State<database::DatabaseState>,
    Query(params): Query<TokyoStockExchangeImportQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let pool = database::get_connection(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let japanese_file = params
        .japanese_file
        .context("japanese_file is required")
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let english_file = params
        .english_file
        .context("english_file is required")
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let db_repo = TokyoStockExchangeRepositoryImpl::new(pool);
    let file_repo = TokyoStockExchangeFileRepositoryImpl::new(
        PathBuf::from(japanese_file),
        PathBuf::from(english_file),
    );

    let file_repo = Arc::new(file_repo);
    let db_repo = Arc::new(db_repo);

    let use_case = ImportFilesUseCase::new(file_repo, db_repo);

    use_case
        .execute()
        .await
        .map_err(|e| AppError::from(e).status_code())?;

    Ok((StatusCode::OK, ()))
}
