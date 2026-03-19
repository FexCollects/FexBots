use entity::command;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let seed_data = vec![
            "he'll",
            "honk",
            "!ping",
            "!dogfonts",
            "!more moles",
            "!shinyroll",
            "!ambeef",
            "!followage",
            "!tidroll",
            "!sidroll",
            "!markcheck",
            "!whaleroll",
            "!tileroll ",
            "!moleroll",
            "!tid",
        ];

        for name in seed_data {
            let model = command::ActiveModel {
                name: Set(name.to_string()),
                ..Default::default()
            };
            model.insert(db).await?;
        }

        println!("Command table seeded successfully.");
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let seed_data = vec![
            "he'll",
            "honk",
            "!ping",
            "!dogfonts",
            "!more moles",
            "!shinyroll",
            "!ambeef",
            "!followage",
            "!tidroll",
            "!sidroll",
            "!markcheck",
            "!whaleroll",
            "!tileroll ",
            "!moleroll",
            "!tid",
        ];

        command::Entity::delete_many()
            .filter(command::Column::Name.is_in(seed_data))
            .exec(db)
            .await?;

        println!("Command seeded data removed.");
        Ok(())
    }
}
