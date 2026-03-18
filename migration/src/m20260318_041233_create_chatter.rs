use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("chatter")
                    .if_not_exists()
                    .col(integer("id").not_null().primary_key())
                    .col(string("name").not_null())
                    .col(integer("tid").not_null().default(0))
                    .col(integer("sid").not_null().default(0))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("chatter").to_owned())
            .await
    }
}
