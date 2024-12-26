use super::_entities::pools::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
pub type Pools = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
