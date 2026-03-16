pub use sea_orm_migration::prelude::*;

mod m20220120_000001_create_post_table;
mod m20220120_000002_seed_posts;
mod m20260315_051110_create_streamer;
mod m20260315_051846_seed_streamer;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220120_000001_create_post_table::Migration),
            Box::new(m20220120_000002_seed_posts::Migration),
            Box::new(m20260315_051110_create_streamer::Migration),
            Box::new(m20260315_051846_seed_streamer::Migration),
        ]
    }
}
