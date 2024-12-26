use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Pools::Table)
                    .col(pk_auto(Pools::Id))
                    .col(string_null(Pools::PoolId))
                    .col(timestamp_null(Pools::StartTime))
                    .col(timestamp_null(Pools::EndTime))
                    .col(timestamp_null(Pools::DrawnTime))
                    .col(timestamp_null(Pools::CreatedTime))
                    .col(string_null(Pools::CreatedBy))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Pools::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Pools {
    Table,
    Id,
    PoolId,
    StartTime,
    EndTime,
    DrawnTime,
    CreatedTime,
    CreatedBy,
}
