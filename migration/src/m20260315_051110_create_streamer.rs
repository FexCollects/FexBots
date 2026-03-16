use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("streamer")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(string("name").not_null())
                    .col(string("broadcaster_user_id").not_null())
                    .col(boolean("shiny_roll_enabled").not_null().default(true))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("streamer").to_owned())
            .await
    }
}
