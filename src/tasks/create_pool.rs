// use chrono::{DateTime, Utc};

use loco_rs::prelude::*;

use sui_config::{sui_config_dir, SUI_KEYSTORE_FILENAME};
use sui_json_rpc_types::SuiTransactionBlockResponse;
use sui_keys::keystore::FileBasedKeystore;

use crate::{
    models::contracts,
    tools::{
        ptb::build_create_lottery_pool_pt,
        util::{call_function, fetch_coin, get_millis_after_1_day, setup_and_write},
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
        // TODO: Create pool
        let resp = create_pool(
            app_context,
            "daily",
            "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
            vars,
        )
        .await
        .map_err(|e| anyhow::anyhow!(e));

        match resp {
            Ok(resp) => {
                dbg!("resp: {}", resp.object_changes);
            }
            Err(e) => {
                dbg!("Error: {}", e);
            }
        }

        println!("Task CreatePool finished");
        Ok(())
    }
}

/// Create a pool with current timestamp, and price is 1 SUI.
/// the contract info is from the latest contract in the database.
///
/// # Errors
///
/// When the contract is not found in the database
pub async fn create_pool(
    ctx: &AppContext,
    pool_type: &str,
    coin_type: &str,
    _vars: &task::Vars,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let (client, active_address, _recipient) = setup_and_write().await?;

    let coin = fetch_coin(&client, active_address)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Failed to fetch coin"))?;

    let contract = contracts::Model::find_latest(&ctx.db).await?;

    let package_id = contract.package_id;
    let registry_id = contract.registry_id;
    let registry_initial_version = u64::from(contract.registry_initial_version.unsigned_abs());

    let create_cap_id = contract.create_cap_id;
    // let registry_initial_version = 309_765_686;
    // let draw_cap_id = "0xbbc3b417fa1d9e8babca85bd0422b2e11645b6c652796d219c9923412fbc29ce".to_string();

    // let package_id = "0x01cb20532799748945d18ed656b6e3af9726d0067a316796f150beb736793bd6".to_string();
    // let registry_id = "0x952569689168ac41183bf1c9028034d2bf437ad817c7e6a21a155a5e95506fe7".to_string();
    // let registry_initial_version = 309_765_686;
    // let create_cap_id = "0x84244e7ac30b25e71e99a1eb0b63dbb973bc0cfdb0b933a5a1ee270d4e4f6b97".to_string();

    let end_time = get_millis_after_1_day();

    let create_pool_pt = build_create_lottery_pool_pt(
        &client,
        &package_id,
        &create_cap_id,
        &registry_id,
        registry_initial_version,
        1_000_000_000,
        end_time,
        pool_type,
        coin_type,
    )
    .await?;

    let keystore = FileBasedKeystore::new(&sui_config_dir()?.join(SUI_KEYSTORE_FILENAME))?;
    let gas_budget = 10_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;

    call_function(
        &client,
        keystore,
        // set_fee_rate_pt,
        // draw_pool_pt,
        create_pool_pt,
        vec![coin.object_ref()],
        gas_budget,
        gas_price,
        active_address,
    )
    .await

    // let create_pool_pt = build_create_lottery_pool_pt(
    //     &client,
    //     "0x008e376008022ef5c0ce479072d93533b5b28222159e111806d38168e86a8f50",
    //     "0xc09adf5e3984f1bcf14d559a80ed6d3a1f50f6fbfc4d4d084bb6e9136caff22f",
    //     "0x1ab10df973a6020f3cf158bbc095337229e6340b0a51191f4d6d1fa3e80e7095",
    //     289_569_513,
    //     100_000_000,
    //     1_737_165_600_000,
    //     "daily",
    //     "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
    // )
    // .await?;

    // let keystore = FileBasedKeystore::new(&sui_config_dir()?.join(SUI_KEYSTORE_FILENAME))?;
    // let gas_budget = 10_000_000;
    // let gas_price = client.read_api().get_reference_gas_price().await?;

    // let resp = call_function(
    //     &client,
    //     keystore,
    //     // set_fee_rate_pt,
    //     // draw_pool_pt,
    //     create_pool_pt,
    //     vec![coin.object_ref()],
    //     gas_budget,
    //     gas_price,
    //     active_address,
    // )
    // .await?;

    // dbg!("{}", resp.object_changes.clone());

    // Ok(resp)
}
