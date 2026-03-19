use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("chatter_command")
                    .if_not_exists()
                    .col(integer("chatter_id").not_null())
                    .col(integer("command_id").not_null())
                    .col(integer("count").not_null().default(0))
                    .primary_key(
                        Index::create()
                                .name("pk-chatter_command")
                                .col("chatter_id")
                                .col("command_id")
                                .primary(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk-chatter_command-chatter_id-id")
                            .from_tbl("chatter_command")
                            .from_col("chatter_id")
                            .to_tbl("chatter")
                            .to_col("id"),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk-chatter_command-command_id-id")
                            .from_tbl("chatter_command")
                            .from_col("command_id")
                            .to_tbl("command")
                            .to_col("id"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("chatter_command").to_owned())
            .await
    }
}
