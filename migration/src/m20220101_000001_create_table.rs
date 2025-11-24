use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("categories")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(string("name"))
                    .col(string("slug"))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table("articles")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(string("title"))
                    .col(string("slug"))
                    .col(string("body"))
                    .col(string("slug"))
                    .col(timestamp("created_at"))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("categories").to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table("articles").to_owned())
            .await?;
        Ok(())
    }
}
