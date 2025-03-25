use crate::domain::tokyo_stock_exchange::repository::{
    TokyoStockExchangeAttributesQueryRepository, TokyoStockExchangeRepository,
};
use std::sync::Arc;

pub struct ImportFilesUseCase {
    file_repository: Arc<dyn TokyoStockExchangeAttributesQueryRepository>,
    repository: Arc<dyn TokyoStockExchangeRepository>,
}

impl ImportFilesUseCase {
    pub fn new(
        file_repository: Arc<dyn TokyoStockExchangeAttributesQueryRepository>,
        repository: Arc<dyn TokyoStockExchangeRepository>,
    ) -> Self {
        Self {
            file_repository,
            repository,
        }
    }

    pub async fn execute(&self) -> anyhow::Result<()> {
        let files = self.file_repository.find_all().await?;
        for file in files {
            let tse = self.repository.find_by_local_code(&file.local_code).await?;

            match tse {
                Some(tse) => {
                    self.repository.save(&tse).await?;
                }
                None => {
                    self.repository.create(&file).await?;
                }
            }
        }
        Ok(())
    }
}
