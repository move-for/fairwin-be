//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "contracts")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i32,
    pub package_id: String,
    pub registry_id: String,
    pub network: String,
    pub is_active: bool,
    pub version: i32,
    pub vault_id: String,
    pub create_cap_id: String,
    pub registry_initial_version: i32,
    pub draw_cap_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::pools::Entity")]
    Pools,
}

impl Related<super::pools::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Pools.def()
    }
}
