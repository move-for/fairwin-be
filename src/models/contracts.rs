use loco_rs::prelude::*;

pub use super::_entities::contracts::{self, ActiveModel, Entity, Model};
use sea_orm::{entity::prelude::*, QueryOrder};
pub type Contracts = Entity;

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

/// implement your read-oriented logic here
impl Model {
    /// Find a contract by `package_id`
    ///
    /// # Errors
    //
    // When the contract is not found in the database
    pub async fn find_by_package_id(
        db: &DatabaseConnection,
        package_id: &str,
    ) -> ModelResult<Self> {
        let contract = contracts::Entity::find()
            .filter(contracts::Column::PackageId.eq(package_id))
            .one(db)
            .await?;
        contract.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Find a contract by `registry_id`
    ///
    /// # Errors
    ///
    /// When the contract is not found in the database
    pub async fn find_by_registry_id(
        db: &DatabaseConnection,
        registry_id: &str,
    ) -> ModelResult<Self> {
        let contract = contracts::Entity::find()
            .filter(contracts::Column::RegistryId.eq(registry_id))
            .one(db)
            .await?;
        contract.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Find latest contract
    ///
    /// # Errors
    ///
    /// When the contract is not found in the database
    pub async fn find_latest(db: &DatabaseConnection, network: &str) -> ModelResult<Self> {
        let contract = contracts::Entity::find()
            .filter(contracts::Column::Network.eq(network))
            .order_by_desc(contracts::Column::Id)
            .one(db)
            .await?;
        contract.ok_or_else(|| ModelError::EntityNotFound)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
