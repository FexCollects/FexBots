pub use sea_orm_migration::prelude::*;

mod m20220120_000001_create_post_table;
mod m20220120_000002_seed_posts;
mod m20260318_041233_create_chatter;
mod m20260318_051448_create_command;
mod m20260318_051631_seed_command;
mod m20260319_023840_create_chatter_command;
mod m20260329_224109_seed_metronome_command;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220120_000001_create_post_table::Migration),
            Box::new(m20220120_000002_seed_posts::Migration),
            Box::new(m20260318_041233_create_chatter::Migration),
            Box::new(m20260318_051448_create_command::Migration),
            Box::new(m20260318_051631_seed_command::Migration),
            Box::new(m20260319_023840_create_chatter_command::Migration),
            Box::new(m20260329_224109_seed_metronome_command::Migration),
        ]
    }
}
