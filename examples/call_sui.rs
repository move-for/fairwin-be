use fairwin::tools::{
    ptb::build_create_lottery_pool_pt,
    util::{call_function, fetch_coin, setup_and_write},
};

use sui_keys::keystore::FileBasedKeystore;

use sui_config::{sui_config_dir, SUI_KEYSTORE_FILENAME};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let (client, active_address, _recipient) = setup_and_write().await?;

    let coin = fetch_coin(&client, active_address).await?.unwrap();

    // let set_fee_rate_pt = build_set_fee_rate_pt(
    //     &client,
    //     "0x008e376008022ef5c0ce479072d93533b5b28222159e111806d38168e86a8f50",
    //     "0x1f275ca984f392eaeee7a23c59c909b678c2f34233dffb9ffae9a89c7b340a31",
    //     "0x1ab10df973a6020f3cf158bbc095337229e6340b0a51191f4d6d1fa3e80e7095",
    //     289569513,
    //     3,
    // )
    // .await?;
    // let draw_pool_pt = build_draw_lottery_pool_pt(
    //     &client,
    //     "0x008e376008022ef5c0ce479072d93533b5b28222159e111806d38168e86a8f50",
    //     "0xf4c53f7dfb422698035f99973e351e75c77aa631e7ca4f3f506c806913856a65",
    //     "0x1ab10df973a6020f3cf158bbc095337229e6340b0a51191f4d6d1fa3e80e7095",
    //     289569513,
    //     "0xff9c86c7767c0f97ba7d77dbbd312a511cfe7ef4d6ae5562d02a589df9a642a9",
    //     "0xe7b4c41fb712e689fd699126f433bee351beaa37157c166f854e11315c51488b",
    //     "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
    // )
    // .await?;

    let create_pool_pt = build_create_lottery_pool_pt(
        &client,
        "0x008e376008022ef5c0ce479072d93533b5b28222159e111806d38168e86a8f50",
        "0xc09adf5e3984f1bcf14d559a80ed6d3a1f50f6fbfc4d4d084bb6e9136caff22f",
        "0x1ab10df973a6020f3cf158bbc095337229e6340b0a51191f4d6d1fa3e80e7095",
        289_569_513,
        100_000_000,
        1_737_165_600_000,
        "daily",
        "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
    )
    .await?;

    let keystore = FileBasedKeystore::new(&sui_config_dir()?.join(SUI_KEYSTORE_FILENAME))?;
    let gas_budget = 10_000_000;
    let gas_price = client.read_api().get_reference_gas_price().await?;

    let resp = call_function(
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
    .await?;

    dbg!("{}", resp.object_changes);

    Ok(())
}
