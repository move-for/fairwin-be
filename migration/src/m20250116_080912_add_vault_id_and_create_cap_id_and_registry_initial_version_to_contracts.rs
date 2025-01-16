use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        add_column(m, "contracts", "vault_id", ColType::String).await?;
        add_column(m, "contracts", "create_cap_id", ColType::String).await?;
        add_column(m, "contracts", "registry_initial_version", ColType::Integer).await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        remove_column(m, "contracts", "vault_id").await?;
        remove_column(m, "contracts", "create_cap_id").await?;
        remove_column(m, "contracts", "registry_initial_version").await?;
        Ok(())
    }
}
