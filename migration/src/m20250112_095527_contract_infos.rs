use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(ContractInfos::Table)
                    .col(pk_auto(ContractInfos::Id))
                    .col(string(ContractInfos::PackageId))
                    .col(string(ContractInfos::RegistryId))
                    .col(string(ContractInfos::Network))
                    .col(boolean(ContractInfos::IsActive))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ContractInfos::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ContractInfos {
    Table,
    Id,
    PackageId,
    RegistryId,
    Network,
    IsActive,
    
}

