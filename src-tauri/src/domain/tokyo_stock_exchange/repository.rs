use crate::domain::shared_kernel::id::Id;

use super::{
    local_code::LocalCode,
    tokyo_stock_exchange::{TokyoStockExchange, TokyoStockExchangeAttributes},
};

pub trait TokyoStockExchangeRepository:
    TokyoStockExchangeQueryRepository + TokyoStockExchangeCommandRepository
{
}

pub trait TokyoStockExchangeQueryRepository {
    async fn find_by_id(&self, id: &Id) -> anyhow::Result<Option<TokyoStockExchange>>;
    async fn find_by_local_code(
        &self,
        local_code: &LocalCode,
    ) -> anyhow::Result<Option<TokyoStockExchange>>;
    async fn find_all(&self) -> anyhow::Result<Vec<TokyoStockExchange>>;
}

pub trait TokyoStockExchangeCommandRepository {
    async fn create(
        &self,
        tokyo_stock_exchange: &TokyoStockExchangeAttributes,
    ) -> anyhow::Result<TokyoStockExchange>;
    async fn save(
        &self,
        tokyo_stock_exchange: &TokyoStockExchange,
    ) -> anyhow::Result<TokyoStockExchange>;
    async fn delete(&self, id: &Id) -> anyhow::Result<()>;
}
