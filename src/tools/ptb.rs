use std::str::FromStr;

use sui_sdk::{
    types::{
        base_types::ObjectID,
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        transaction::{CallArg, ObjectArg, ProgrammableTransaction},
        Identifier, TypeTag,
    },
    SuiClient,
};

use super::util::get_owned_object_arg;

/// Build a programmable transaction to set the fee rate
///
/// # Errors:
///
/// When Sui network is not connected
/// Or when the registry id is not a valid object id
/// Or when the new rate is not a valid u64
pub async fn build_set_fee_rate_pt(
    client: &SuiClient,
    pkg_id: &str,
    admin_id: &str,
    registry_id: &str,
    registry_initial_version: u64,
    new_rate: u64,
) -> Result<ProgrammableTransaction, anyhow::Error> {
    let admin_arg = get_owned_object_arg(client, admin_id).await?;

    let registry_arg = CallArg::Object(ObjectArg::SharedObject {
        id: registry_id.parse()?,
        initial_shared_version: registry_initial_version.into(),
        mutable: true,
    });

    let new_rate_arg = CallArg::Pure(bcs::to_bytes(&new_rate)?);

    // Call `set_fee_rate_api` function
    build_pt(
        pkg_id,
        "main",
        "set_fee_rate_api",
        vec![],
        vec![admin_arg, registry_arg, new_rate_arg],
    )
}

/// Build create lottery pool ptb   // TODO:
///
/// # Errors:
///
/// When Sui network is not connected
pub async fn build_create_lottery_pool_pt(
    client: &SuiClient,
    pkg_id: &str,
    create_id: &str,
    registry_id: &str,
    registry_initial_version: u64,
    price: u64,
    end_time_ms: u64,
    pool_type: &str,
    type_tag: &str,
) -> Result<ProgrammableTransaction, anyhow::Error> {
    let create_arg = get_owned_object_arg(client, create_id).await?;
    let registry_arg = CallArg::Object(ObjectArg::SharedObject {
        id: registry_id.parse()?,
        initial_shared_version: registry_initial_version.into(),
        mutable: true,
    });

    let price_arg = CallArg::Pure(bcs::to_bytes(&price)?);
    let end_time_arg = CallArg::Pure(bcs::to_bytes(&end_time_ms)?);
    let pool_type_arg = CallArg::Pure(bcs::to_bytes(&pool_type)?);

    let tpye_arg = TypeTag::from_str(type_tag)?;

    build_pt(
        pkg_id,
        "main",
        "create_pool_api",
        vec![tpye_arg],
        vec![
            create_arg,
            registry_arg,
            price_arg,
            end_time_arg,
            pool_type_arg,
        ],
    )
}

/// Build draw lottery pool ptb   
///
/// Errors:
///
/// When Sui network is not connected
/// Or when the draw id is not a valid object id
/// Or when the registry id is not a valid object id
/// Or when the vault id is not a valid object id
/// Or when the pool id is not a valid object id
/// Or when the type tag is not a valid type tag
pub async fn build_draw_lottery_pool_pt(
    client: &SuiClient,
    pkg_id: &str,
    draw_id: &str,
    registry_id: &str,
    registry_initial_version: u64,
    vault_id: &str,
    pool_id: &str,
    type_tag: &str,
) -> Result<ProgrammableTransaction, anyhow::Error> {
    let draw_arg = get_owned_object_arg(client, draw_id).await?;
    let registry_arg = CallArg::Object(ObjectArg::SharedObject {
        id: registry_id.parse()?,
        initial_shared_version: registry_initial_version.into(),
        mutable: true,
    });
    let vault_arg = get_owned_object_arg(client, vault_id).await?;

    let pool_arg = CallArg::Pure(bcs::to_bytes(&ObjectID::from_hex_literal(pool_id)?)?);
    let random_arg = CallArg::Object(ObjectArg::SharedObject {
        id: "0x0000000000000000000000000000000000000000000000000000000000000008".parse()?,
        initial_shared_version: 43_342_337.into(),
        mutable: false,
    });
    let clock_arg = CallArg::Object(ObjectArg::SharedObject {
        id: "0x0000000000000000000000000000000000000000000000000000000000000006".parse()?,
        initial_shared_version: 1.into(),
        mutable: false,
    });

    let tpye_arg = TypeTag::from_str(type_tag)?;

    build_pt(
        pkg_id,
        "main",
        "draw_pool_api",
        vec![tpye_arg],
        vec![
            draw_arg,
            registry_arg,
            vault_arg,
            pool_arg,
            random_arg,
            clock_arg,
        ],
    )
}

/// Build a programmable transaction with given arguments
///
/// # Errors:
///
/// When Sui network is not connected
/// Or when the package id is not a valid object id
/// Or when the module is not a valid identifier
/// Or when the function is not a valid identifier
/// Or when the type arguments are not a valid type tag
/// Or when the call arguments are not a valid call arg
pub fn build_pt(
    pkg_id: &str,
    module: &str,
    function: &str,
    type_arguments: Vec<TypeTag>,
    call_args: Vec<CallArg>,
) -> Result<ProgrammableTransaction, anyhow::Error> {
    let mut ptb = ProgrammableTransactionBuilder::new();

    let package = ObjectID::from_hex_literal(pkg_id)?;

    let module = Identifier::new(module)?;
    let function = Identifier::new(function)?;

    ptb.move_call(package, module, function, type_arguments, call_args)?;

    Ok(ptb.finish())
}
