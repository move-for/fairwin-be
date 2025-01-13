#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250113_012646_contracts;
mod m20250113_105404_pools;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250113_012646_contracts::Migration),
            Box::new(m20250113_105404_pools::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
