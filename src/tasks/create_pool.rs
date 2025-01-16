use loco_rs::prelude::*;

pub struct CreatePool;
#[async_trait]
impl Task for CreatePool {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "CreatePool".to_string(),
            detail: "Task generator for Create pool".to_string(),
        }
    }
    async fn run(&self, _app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        // TODO: Create pool
        println!("Task CreatePool finished");
        Ok(())
    }
}
