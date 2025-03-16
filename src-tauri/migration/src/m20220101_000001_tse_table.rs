use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(timestamps(
                Table::create()
                    .table(TokyoStockExchanges::Table)
                    .if_not_exists()
                    .col(pk_auto(TokyoStockExchanges::Id))
                    .col(string(TokyoStockExchanges::LocalCode).unique_key())
                    .col(text(TokyoStockExchanges::EnglishName))
                    .col(text(TokyoStockExchanges::JapaneseName))
                    .col(string(TokyoStockExchanges::Section))
                    .col(small_integer_null(TokyoStockExchanges::Sector33Code))
                    .col(small_integer_null(TokyoStockExchanges::Sector17Code))
                    .col(small_integer_null(TokyoStockExchanges::SizeCode))
                    .to_owned(),
            ))
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tokyo_stock_exchanges_local_code")
                    .table(TokyoStockExchanges::Table)
                    .col(TokyoStockExchanges::LocalCode)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tokyo_stock_exchanges_english_name")
                    .table(TokyoStockExchanges::Table)
                    .col(TokyoStockExchanges::EnglishName)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tokyo_stock_exchanges_japanese_name")
                    .table(TokyoStockExchanges::Table)
                    .col(TokyoStockExchanges::JapaneseName)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tokyo_stock_exchanges_section")
                    .table(TokyoStockExchanges::Table)
                    .col(TokyoStockExchanges::Section)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_index(
                Index::drop()
                    .name("idx_tokyo_stock_exchanges_local_code")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_tokyo_stock_exchanges_english_name")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_tokyo_stock_exchanges_japanese_name")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_tokyo_stock_exchanges_section")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(TokyoStockExchanges::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum TokyoStockExchanges {
    Table,
    Id,
    LocalCode,
    EnglishName,
    JapaneseName,
    Section,
    Sector33Code,
    Sector17Code,
    SizeCode,
}
