use std::time::Duration;

use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::users::users;

pub struct Worker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WorkerArgs {}

#[async_trait]
impl BackgroundWorker<WorkerArgs> for Worker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        println!("=================ReportWorker=======================");
        // TODO: Some actual work goes here...
        dbg!("#############");
        println!("Testing workder to user: {args:?}");

        tokio::time::sleep(Duration::from_millis(2_000)).await;

        let all = users::Entity::find()
            .all(&self.ctx.db)
            .await
            .map_err(Box::from)?;

        for user in &all {
            dbg!("User: {:?}", user.id);
        }

        println!("All users: {:?}", all);
        Ok(())
    }
}
