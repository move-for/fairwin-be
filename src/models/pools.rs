use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;
use sea_orm::Condition;

pub use super::_entities::pools::{self, ActiveModel, Entity, Model};

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

// implement your read-oriented logic here
impl Model {
    /// Find the latest undrawn & expired pool
    ///
    /// # Errors
    ///
    /// when query databse error
    pub async fn find_latest_undrawn(db: &DbConn) -> ModelResult<Option<Self>> {
        let _now: DateTimeWithTimeZone = chrono::Utc::now().into();
        let pool = Entity::find()
            .filter(
                Condition::all().add(pools::Column::DrawnTime.is_null()), // .add(pools::Column::EndTime.lt(now)),
            )
            .one(db)
            .await?;

        Ok(pool)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
