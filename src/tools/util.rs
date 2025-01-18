//! Call a contract function
//!
//!  1. Create a `SuiClient` and sender address
//!  2. Get a gas coin object
//!  3. Building a `ProgrammableTransactionBuilder` (ptb)
//!  4. Add arguments to the ptb
//!  5. Sign the ptb
//!  6. Send the ptb to Sui network
//!  7. Parse the response
//!  8. save the object id to the database
//!

use chrono::{DateTime, Utc};

use futures::{future, stream::StreamExt};

use tracing::info;

use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};

use sui_sdk::{
    rpc_types::SuiObjectDataOptions,
    sui_client_config::{SuiClientConfig, SuiEnv},
    // wallet::Wallet,
    // faucet::FaucetClient,
    types::{
        base_types::{ObjectRef, SuiAddress},
        crypto::SignatureScheme::ED25519,
        quorum_driver_types::ExecuteTransactionRequestType,
        transaction::{CallArg, ObjectArg, ProgrammableTransaction, Transaction, TransactionData},
    },
    wallet_context::WalletContext,
    SuiClient,
    SuiClientBuilder,
};

use sui_json_rpc_types::{Coin, SuiObjectData};

use sui_config::{
    sui_config_dir, Config, PersistedConfig, SUI_CLIENT_CONFIG, SUI_KEYSTORE_FILENAME,
};

use shared_crypto::intent::Intent;
use sui_json_rpc_types::{SuiTransactionBlockResponse, SuiTransactionBlockResponseOptions};

/// Call entry function for Sui network of Move
///
/// # Errors
///
/// When Sui network is not connected
pub async fn call_function(
    client: &SuiClient,
    keystore: FileBasedKeystore,
    pt: ProgrammableTransaction,
    gas_payment: Vec<ObjectRef>,
    gas_budget: u64,
    gas_price: u64,
    sender: SuiAddress,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let tx_data = TransactionData::new_programmable(sender, gas_payment, pt, gas_budget, gas_price);

    let signature = keystore.sign_secure(&sender, &tx_data, Intent::sui_transaction())?;

    let resp = client
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(tx_data, vec![signature]),
            SuiTransactionBlockResponseOptions::full_content(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    Ok(resp)
}

/// Get object by id
///
/// # Errors
///
/// When Sui network is not connected
pub async fn get_object_by_id(
    client: &SuiClient,
    id: &str,
) -> Result<SuiObjectData, anyhow::Error> {
    Ok(client
        .read_api()
        .get_object_with_options(id.parse()?, SuiObjectDataOptions::bcs_lossless())
        .await?
        .into_object()?)
}

/// Get owned object arg
///
/// # Errors
///
/// When Sui network is not connected or when the object id is not a valid object id
pub async fn get_owned_object_arg(client: &SuiClient, id: &str) -> Result<CallArg, anyhow::Error> {
    let object = get_object_by_id(client, id).await?;
    Ok(CallArg::Object(ObjectArg::ImmOrOwnedObject((
        object.object_id,
        object.version,
        object.digest,
    ))))
}

/// Fetch coin from client
///
/// # Errors
///
/// When Sui network is not connected
pub async fn fetch_coin(
    client: &SuiClient,
    address: SuiAddress,
) -> Result<Option<Coin>, anyhow::Error> {
    let coin_type = "0x2::sui::SUI".to_string();
    let coins_stream = client
        .coin_read_api()
        .get_coins_stream(address, Some(coin_type));

    let mut coins = coins_stream
        .skip_while(|c| future::ready(c.balance < 5_000_000))
        .boxed();

    let coin = coins.next().await;

    // let coins = client
    //     .coin_read_api()
    //     .get_coins(active_address, None, None, None)
    //     .await?;
    // let coin = coins.data.into_iter().next().unwrap();

    Ok(coin)
}

/// retrieve wallet from config
///
/// # Errors
///
/// When Sui network is not connected
pub fn retrieve_wallet() -> Result<WalletContext, anyhow::Error> {
    let wallet_config = sui_config_dir()?.join(SUI_CLIENT_CONFIG);
    let keystore_path = sui_config_dir()?.join(SUI_KEYSTORE_FILENAME);

    // check if wallet_config exists and if not, create a wallet and sui client config
    if !keystore_path.exists() {
        let keystore = FileBasedKeystore::new(&keystore_path)?;
        keystore.save()?;
    }

    if !wallet_config.exists() {
        let keystore = FileBasedKeystore::new(&keystore_path)?;
        let mut client_config = SuiClientConfig::new(keystore.into());

        client_config.add_env(SuiEnv::testnet());
        // client_config.add_env(SuiEnv::mainnet());
        client_config.add_env(SuiEnv::devnet());

        if client_config.active_env.is_none() {
            client_config.active_env = client_config.envs.first().map(|env| env.alias.clone());
        }

        client_config.save(&wallet_config)?;
        info!("Client config file is stored in {:?}", &wallet_config);
    }

    let mut keystore = FileBasedKeystore::new(&keystore_path)?;
    let mut client_config: SuiClientConfig = PersistedConfig::read(&wallet_config)?;

    let default_address = if let Some(address) = keystore.addresses().first() {
        *address
    } else {
        keystore
            .generate_and_add_new_key(ED25519, None, None, None)?
            .0
    };

    if keystore.addresses().len() < 2 {
        keystore.generate_and_add_new_key(ED25519, None, None, None)?;
    }

    client_config.active_address = Some(default_address);
    client_config.save(&wallet_config)?;

    let wallet = WalletContext::new(
        &wallet_config,
        Some(std::time::Duration::from_secs(60)),
        None,
    )?;

    Ok(wallet)
}

/// Setup sui client and get active address
///
/// # Errors
///
/// When Sui network is not connected
pub async fn setup_for_read() -> Result<(SuiClient, SuiAddress), anyhow::Error> {
    let client = SuiClientBuilder::default().build_testnet().await?;

    println!("Sui testnet version is: {}", client.api_version());

    let mut wallet = retrieve_wallet()?;

    // assert_eq!(wallet.get_addresses().len(), 2);

    let active_address = wallet.active_address()?;

    // println!("Active address is: {active_address}");

    Ok((client, active_address))
}

/// Setup sui config to build client and get addresses
///
/// # Errors
///
/// When Sui network is not connected
pub async fn setup_and_write() -> Result<(SuiClient, SuiAddress, SuiAddress), anyhow::Error> {
    let (client, active_address) = setup_for_read().await?;

    // let coin = fetch_coin(&client, active_address).await?;

    // if coin.is_none() {
    //     request_tokens_from_faucet(&client, active_address).await?;
    // }

    let wallet = retrieve_wallet()?;

    let addresses = wallet.get_addresses();

    let addresses = addresses
        .into_iter()
        .filter(|a| a != &active_address)
        .collect::<Vec<_>>();

    let recipient = addresses
        .first()
        .ok_or_else(|| anyhow::anyhow!("No addresses found"))?;

    Ok((client, active_address, *recipient))
}

/// Get current time in milliseconds
#[must_use]
pub fn get_current_millis() -> u64 {
    let now = Utc::now();
    now.timestamp_millis().unsigned_abs()
}

/// Get milleseconds after 1 day
#[must_use]
pub fn get_millis_after_1_day() -> u64 {
    get_datetime_after_1_day().timestamp_millis().unsigned_abs()
}

/// Calculate a datetime after 1 day
#[must_use]
pub fn get_datetime_after_1_day() -> DateTime<Utc> {
    let now = Utc::now();
    now.checked_add_signed(chrono::TimeDelta::days(1))
        .unwrap_or(now)
}
