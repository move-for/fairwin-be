use sea_orm::entity::prelude::*;
use super::_entities::pools::{ActiveModel, Entity};
pub type Pools = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
