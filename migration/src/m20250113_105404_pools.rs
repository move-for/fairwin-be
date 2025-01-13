use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "pools",
            &[
                ("pool_id", ColType::String),
                ("price", ColType::Integer),
                ("type_name", ColType::String),
                ("pool_type", ColType::String),
                ("start_time", ColType::TimestampWithTimeZone),
                ("end_time", ColType::TimestampWithTimeZone),
                ("drawn_time", ColType::TimestampWithTimeZoneNull),
                ("lucky_number", ColType::StringNull),
                ("round", ColType::IntegerNull),
                ("epoch", ColType::IntegerNull),
                ("is_active", ColType::Boolean),
            ],
            &[("contract", "")],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "pools").await
    }
}
