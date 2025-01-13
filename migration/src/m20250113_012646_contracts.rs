use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "contracts",
            &[
                ("package_id", ColType::String),
                ("registry_id", ColType::String),
                ("network", ColType::String),
                ("is_active", ColType::Boolean),
                ("version", ColType::Integer),
            ],
            &[],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "contracts").await
    }
}
