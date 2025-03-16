use std::ops::Deref as _;

use anyhow::Context as _;
use async_trait::async_trait;

use crate::{
    domain::{
        shared_kernel::id::Id,
        tokyo_stock_exchange::{
            local_code::LocalCode,
            repository::{TokyoStockExchangeCommandRepository, TokyoStockExchangeQueryRepository},
            section::Section,
            sector17_code::Sector17Code,
            sector33_code::Sector33Code,
            size_code::SizeCode,
            tokyo_stock_exchange::{TokyoStockExchange, TokyoStockExchangeAttributes},
        },
    },
    infrastructure::persistence::entities::{prelude::TokyoStockExchanges, tokyo_stock_exchanges},
};
use sea_orm::{
    ActiveModelTrait as _, ActiveValue::Set, ColumnTrait as _, DatabaseConnection, EntityTrait,
    QueryFilter as _, TransactionTrait as _, TryIntoModel,
};

pub struct TokyoStockExchangeRepositoryImpl {
    conn: DatabaseConnection,
}

impl TokyoStockExchangeRepositoryImpl {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    async fn to_domain(
        &self,
        model: tokyo_stock_exchanges::Model,
    ) -> anyhow::Result<TokyoStockExchange> {
        Ok(TokyoStockExchange {
            id: Id::from(model.id),
            local_code: LocalCode::try_from(model.local_code)?,
            english_name: model.english_name.into(),
            japanese_name: model.japanese_name.into(),
            section: Section::try_from(model.section)?,
            sector33_code: Sector33Code::try_from(model.sector33_code)?,
            sector17_code: Sector17Code::try_from(model.sector17_code)?,
            size_code: SizeCode::try_from(model.size_code)?,
            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
        })
    }

    async fn create_active_model(
        &self,
        domain: &TokyoStockExchangeAttributes,
    ) -> tokyo_stock_exchanges::ActiveModel {
        tokyo_stock_exchanges::ActiveModel {
            local_code: Set(domain.local_code.to_string()),
            english_name: Set(domain.english_name.deref().clone()),
            japanese_name: Set(domain.japanese_name.deref().clone()),
            section: Set(domain.section.to_string()),
            sector33_code: Set(domain.sector33_code.deref().clone()),
            sector17_code: Set(domain.sector17_code.deref().clone()),
            size_code: Set(domain.size_code.deref().clone()),
            ..Default::default()
        }
    }

    async fn update_active_model(
        &self,
        domain: &TokyoStockExchange,
    ) -> tokyo_stock_exchanges::ActiveModel {
        tokyo_stock_exchanges::ActiveModel {
            id: Set(domain.id.deref().clone()),
            local_code: Set(domain.local_code.to_string()),
            english_name: Set(domain.english_name.deref().clone()),
            japanese_name: Set(domain.japanese_name.deref().clone()),
            section: Set(domain.section.to_string()),
            sector33_code: Set(domain.sector33_code.deref().clone()),
            sector17_code: Set(domain.sector17_code.deref().clone()),
            size_code: Set(domain.size_code.deref().clone()),
            ..Default::default()
        }
    }
}

#[async_trait]
impl TokyoStockExchangeQueryRepository for TokyoStockExchangeRepositoryImpl {
    async fn find_by_id(&self, id: &Id) -> anyhow::Result<Option<TokyoStockExchange>> {
        let id = *id.deref();

        let result = TokyoStockExchanges::find_by_id(id)
            .one(&self.conn)
            .await
            .with_context(|| format!("Failed to find tokyo stock exchange by id: {}", id))?;

        match result {
            Some(model) => Ok(Some(self.to_domain(model).await?)),
            None => Ok(None),
        }
    }

    async fn find_by_local_code(
        &self,
        local_code: &LocalCode,
    ) -> anyhow::Result<Option<TokyoStockExchange>> {
        // let local_code: String = local_code.to_string();

        let result = TokyoStockExchanges::find()
            .filter(tokyo_stock_exchanges::Column::LocalCode.eq(local_code.to_string()))
            .one(&self.conn)
            .await
            .with_context(|| {
                format!(
                    "Failed to find tokyo stock exchange by local code: {}",
                    local_code
                )
            })?;

        match result {
            Some(model) => Ok(Some(self.to_domain(model).await?)),
            None => Ok(None),
        }
    }

    async fn find_all(&self) -> anyhow::Result<Vec<TokyoStockExchange>> {
        let models = TokyoStockExchanges::find()
            .all(&self.conn)
            .await
            .context("Failed to find all tokyo stock exchanges")?;

        let mut exchanges = Vec::with_capacity(models.len());
        for model in models {
            exchanges.push(self.to_domain(model).await?);
        }

        Ok(exchanges)
    }
}

#[async_trait]
impl TokyoStockExchangeCommandRepository for TokyoStockExchangeRepositoryImpl {
    async fn create(
        &self,
        tokyo_stock_exchange: &TokyoStockExchangeAttributes,
    ) -> anyhow::Result<TokyoStockExchange> {
        let tnx = self.conn.begin().await?;

        let m = self.create_active_model(tokyo_stock_exchange).await;
        let m = m
            .save(&tnx)
            .await
            .context("Failed to create tokyo stock exchange")?;

        let m = m
            .clone()
            .try_into_model()
            .with_context(|| format!("Failed to convert tokyo stock exchange model: {:?}", &m))?;
        let result = self.to_domain(m).await.context("Failed model to domain")?;

        tnx.commit().await?;
        Ok(result)
    }

    async fn save(
        &self,
        tokyo_stock_exchange: &TokyoStockExchange,
    ) -> anyhow::Result<TokyoStockExchange> {
        let tnx = self.conn.begin().await?;

        let m = self.update_active_model(tokyo_stock_exchange).await;
        let m = m.save(&tnx).await.with_context(|| {
            format!(
                "Failed to save tokyo stock exchange: {}",
                tokyo_stock_exchange.id
            )
        })?;

        let m = m
            .clone()
            .try_into_model()
            .with_context(|| format!("Failed to convert tokyo stock exchange model: {:?}", &m))?;
        let result = self.to_domain(m).await.context("Failed model to domain")?;

        tnx.commit().await?;
        Ok(result)
    }

    async fn delete(&self, id: &Id) -> anyhow::Result<()> {
        tokyo_stock_exchanges::Entity::delete_by_id(**id)
            .exec(&self.conn)
            .await
            .with_context(|| format!("Failed to delete tokyo stock exchange: {}", **id))?;

        Ok(())
    }
}
