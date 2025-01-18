// use chrono::{DateTime, Utc};

use loco_rs::prelude::*;

use sui_config::{sui_config_dir, SUI_KEYSTORE_FILENAME};
use sui_json_rpc_types::ObjectChange;
use sui_keys::keystore::FileBasedKeystore;

use crate::{
    models::{contracts, pools},
    tools::{
        ptb::build_create_lottery_pool_pt,
        util::{
            call_function, fetch_coin, get_datetime_after_1_day, get_millis_after_1_day,
            setup_and_write,
        },
    },
};

pub struct CreatePool;

#[async_trait]
impl Task for CreatePool {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "CreatePool".to_string(),
            detail: "Task generator for Create pool".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        // Create pool
        let resp = create_pool(
            app_context,
            "daily",
            "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
            vars,
        )
        .await;

        match resp {
            Ok(lottery_pool_id) => {
                dbg!("lottery_pool_id: {}", lottery_pool_id);
            }
            Err(e) => {
                dbg!("Error: {}", e);
            }
        }

        println!("Task CreatePool finished");
        Ok(())
    }
}

/// Get object id of `LotteryPool` in `ObjectChange`  , if not exists, return None.
fn get_lottery_pool_id(object_change: ObjectChange) -> Option<String> {
    match object_change {
        ObjectChange::Created {
            object_id,
            object_type,
            ..
        } if object_type.name.as_str() == "LotteryPool" => Some(object_id.to_string()),
        _ => None,
    }
}

/// Get object id of `LotteryPool` in `Option<Vec<ObjectChange>>`  , if not exists, return None.
#[must_use]
pub fn get_lottery_pool_id_from_object_changes(
    object_changes: Option<Vec<ObjectChange>>,
) -> Option<String> {
    object_changes.and_then(|oc| oc.iter().find_map(|oc| get_lottery_pool_id(oc.clone())))
}

/// Create a pool with current timestamp, and price is 1 SUI.
/// the contract info is from the latest contract in the database.
///
/// # Errors
///
/// When the contract is not found in the database
///
async fn create_pool(
    ctx: &AppContext,
    pool_type: &str,
    coin_type: &str,
    _vars: &task::Vars,
) -> Result<String, anyhow::Error> {
    let (client, active_address, _recipient) = setup_and_write().await?;

    let coin = fetch_coin(&client, active_address)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Failed to fetch coin"))?;

    let contract = contracts::Model::find_latest(&ctx.db).await?;

    let package_id = contract.package_id;
    let registry_id = contract.registry_id;
    let registry_initial_version = u64::from(contract.registry_initial_version.unsigned_abs());
    let price = 1_000_000_000;

    let create_cap_id = contract.create_cap_id;

    let end_time = get_datetime_after_1_day();
    let end_time_ms = get_millis_after_1_day();

    let create_pool_pt = build_create_lottery_pool_pt(
        &client,
        &package_id,
        &create_cap_id,
        &registry_id,
        registry_initial_version,
        price,
        end_time_ms,
        pool_type,
        coin_type,
    )
    .await?;

    let keystore = FileBasedKeystore::new(&sui_config_dir()?.join(SUI_KEYSTORE_FILENAME))?;
    let gas_budget = 10_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;

    let resp = call_function(
        &client,
        keystore,
        create_pool_pt,
        vec![coin.object_ref()],
        gas_budget,
        gas_price,
        active_address,
    )
    .await?;

    dbg!("{:#?}", &resp.object_changes);

    if let Some(lottery_pool_id) = get_lottery_pool_id_from_object_changes(resp.object_changes) {
        // build pool
        let pool = pools::ActiveModel {
            pool_id: Set(lottery_pool_id.clone()),
            price: Set(i32::try_from(price)?),
            type_name: Set(coin_type.to_string()),
            pool_type: Set(pool_type.to_string()),
            start_time: Set(chrono::Utc::now().into()),
            end_time: Set(end_time.into()),
            drawn_time: Set(None),
            lucky_number: Set(None),
            round: Set(None),
            epoch: Set(None),
            is_active: Set(true),
            contract_id: Set(contract.id),
            ..Default::default()
        };

        pool.insert(&ctx.db).await?;
        // }
        Ok(lottery_pool_id)
    } else {
        Err(anyhow::anyhow!("Failed to create pool!"))
    }
}
