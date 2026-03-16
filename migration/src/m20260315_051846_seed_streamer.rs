use entity::streamer;
use sea_orm::{ActiveModelTrait, Set};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let seed_data = vec![
            ("FexCollects", "68411561", true),
            ("Magnemite", "106239207", true),
            ("SBCoop", "176723607", true),
            ("skyfishjack", "83193553", true),
            ("LegendLinke", "214333642", true),
            ("BigWiggins", "103539171", true),
            ("yarnity", "861073341", false),
        ];

        for (name, cast_id, shiny_roll) in seed_data {
            let model = streamer::ActiveModel {
                name: Set(name.to_string()),
                broadcaster_user_id: Set(cast_id.to_string()),
                shiny_roll_enabled: Set(shiny_roll),
                ..Default::default()
            };
            model.insert(db).await?;
        }

        println!("Streamer table seeded successfully.");
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        todo!()
    }
}
