use loco_rs::prelude::*;

use sui_config::{sui_config_dir, SUI_KEYSTORE_FILENAME};

use sui_keys::keystore::FileBasedKeystore;

use crate::{
    models::{contracts, pools},
    tasks::create_pool::get_lottery_pool_id_from_object_changes,
    tools::{
        ptb::build_draw_lottery_pool_pt,
        util::{call_function, fetch_coin, setup_and_write},
    },
};

pub struct DrawPool;

#[async_trait]
impl Task for DrawPool {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "draw_pool".to_string(),
            detail: "Task generator".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        match draw_pool(app_context, vars).await {
            Ok(()) => {
                println!("Task DrawPool generated");
                Ok(())
            }
            Err(e) => {
                println!("Error: {e:#?}");
                Err(loco_rs::Error::TaskNotFound("DrawPool".to_string()))
            }
        }
    }
}

/// Draw lottery pool which undrawn
/// 1. Get undrawn pool
/// 2. Draw pool
/// 3. Update pool
/// 4. Update user's lottery pool
///
/// # Errors
///
/// When the pool is not found in the database
async fn draw_pool(ctx: &AppContext, _vars: &task::Vars) -> Result<(), anyhow::Error> {
    let (client, active_address, _recipient) = setup_and_write().await?;

    let coin = fetch_coin(&client, active_address).await?.unwrap();

    let keystore = FileBasedKeystore::new(&sui_config_dir()?.join(SUI_KEYSTORE_FILENAME))?;
    let gas_budget = 10_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;

    let pool = pools::Model::find_latest_undrawn(&ctx.db).await?;

    if let Some(pool) = pool {
        let contract = pool.find_related(contracts::Entity).one(&ctx.db).await?;

        if let Some(contract) = contract {
            dbg!("{:?}", &contract);
            dbg!("{:?}", &pool);

            let vault_id = &contract.vault_id;
            let draw_pool_pt = build_draw_lottery_pool_pt(
                &client,
                &contract.package_id,
                &contract.draw_cap_id,
                &contract.registry_id,
                u64::from(contract.registry_initial_version.unsigned_abs()),
                vault_id,
                &pool.pool_id,
                &pool.type_name,
            )
            .await?;

            dbg!("{:?}", &draw_pool_pt);

            let resp = call_function(
                &client,
                keystore,
                draw_pool_pt,
                vec![coin.object_ref()],
                gas_budget,
                gas_price,
                active_address,
            )
            .await?;

            if get_lottery_pool_id_from_object_changes(resp.object_changes).is_some() {
                let mut pool: pools::ActiveModel = pool.into();
                pool.drawn_time = Set(Some(chrono::Utc::now().into()));
                pool.update(&ctx.db).await?;
            }
            // dbg!("{:?}", &resp);
        }
    }

    Ok(())
}
