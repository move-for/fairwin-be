use loco_rs::prelude::*;

use sui_config::{sui_config_dir, SUI_KEYSTORE_FILENAME};
use sui_json_rpc_types::ObjectChange;
use sui_keys::keystore::FileBasedKeystore;

use crate::{
    models::{contracts, pools},
    tools::{
        ptb::build_draw_lottery_pool_pt,
        util::{call_function, fetch_coin, setup_for_read},
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
async fn draw_pool(ctx: &AppContext, vars: &task::Vars) -> Result<(), anyhow::Error> {
    let network = vars
        .cli_arg("network")
        .map_or_else(|_| "testnet".to_string(), std::string::ToString::to_string);

    let (client, active_address) = setup_for_read(&network).await?;

    let coin = fetch_coin(&client, active_address).await?.unwrap();

    let keystore = FileBasedKeystore::new(&sui_config_dir()?.join(SUI_KEYSTORE_FILENAME))?;
    let gas_budget = 10_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;

    let pool = pools::Model::find_oldest_undrawn(&ctx.db).await?;

    dbg!("{:?}", &pool);

    if let Some(pool) = pool {
        let contract = pool.find_related(contracts::Entity).one(&ctx.db).await?;

        if let Some(contract) = contract {
            dbg!("{:?}", &contract);

            let registry_initial_version =
                u64::from(contract.registry_initial_version.unsigned_abs());
            dbg!("{:?}", &registry_initial_version);

            let vault_id = &contract.vault_id;
            let draw_pool_pt = build_draw_lottery_pool_pt(
                &client,
                &contract.package_id,
                &contract.draw_cap_id,
                &contract.registry_id,
                registry_initial_version,
                vault_id,
                &pool.pool_id,
                &pool.type_name,
            )
            .await?;

            // dbg!("{:?}", &draw_pool_pt);

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

            dbg!("{:?}", &resp.object_changes);

            if get_lottery_pool_id_from_object_changes_mutated(resp.object_changes).is_some() {
                let mut pool: pools::ActiveModel = pool.into();
                pool.drawn_time = Set(Some(chrono::Utc::now().into()));
                pool.is_active = Set(false);

                dbg!("{:?}", &pool);

                let resp = pool.update(&ctx.db).await?;
                dbg!("{:?}", &resp);
            }
        }
    }

    Ok(())
}

/// Get object id of `LotteryPool` in `ObjectChange`  , if not exists, return None.
fn get_lottery_pool_id_from_mutate(object_change: ObjectChange) -> Option<String> {
    match object_change {
        ObjectChange::Mutated {
            object_id,
            object_type,
            ..
        } if object_type.name.as_str() == "LotteryPool" => Some(object_id.to_string()),
        _ => None,
    }
}

/// Get object id of `LotteryPool` in `Option<Vec<ObjectChange>>`  , if not exists, return None.
#[must_use]
pub fn get_lottery_pool_id_from_object_changes_mutated(
    object_changes: Option<Vec<ObjectChange>>,
) -> Option<String> {
    object_changes.and_then(|oc| {
        oc.iter()
            .find_map(|oc| get_lottery_pool_id_from_mutate(oc.clone()))
    })
}
