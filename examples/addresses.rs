use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
// use sui_sdk::types::base_types::SuiAddress;
use anyhow::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    // Get all addresses in keystore
    let keystore_path = PathBuf::from(std::env::var("HOME").unwrap())
        .join(".sui")
        .join("sui_config")
        .join("sui.keystore");

    let keystore = FileBasedKeystore::new(&keystore_path)?;
    let addresses = keystore.addresses();

    // Print all addresses
    println!("Addresses in keystore:");
    for address in addresses {
        dbg!("{:?}", &address);
    }

    Ok(())
}
