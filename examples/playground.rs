use chrono::{DateTime, Utc};
use fairwin::app::App;

#[allow(unused_imports)]
use loco_rs::{cli::playground, prelude::*};

// use fairwin::models::_entities::pools::{self, ActiveModel};
use fairwin::models::_entities::contracts;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    let ctx = playground::<App>().await?;

    // let active_model: pools::ActiveModel = ActiveModel {
    //     pool_id: Set(Some("0x1ab10df973a6020f3cf158bbc095337229e6340b0a51191f4d6d1fa3e80e7095".to_string())),
    //     start_time: Set(Some(DateTime::<Utc>::from_timestamp_millis(1735122600000).unwrap().naive_utc())),
    //     end_time: Set(Some(DateTime::<Utc>::from_timestamp_millis(1735122600000).unwrap().naive_utc())),
    //     drawn_time: Set(Some(DateTime::<Utc>::from_timestamp_millis(1735122600000).unwrap().naive_utc())),
    //     created_time: Set(Some(DateTime::<Utc>::from_timestamp_millis(1735122600000).unwrap().naive_utc())),
    //     created_by: Set(Some("admin".to_string())),
    //     ..Default::default()
    // };
    // active_model.insert(&ctx.db).await.unwrap();

    // let res = articles::Entity::find().all(&ctx.db).await.unwrap();
    // println!("{:?}", res);
    // let res = pools::Entity::find().all(&ctx.db).await.unwrap();
    // println!("{:?}", res);

    let package_id =
        "0x01cb20532799748945d18ed656b6e3af9726d0067a316796f150beb736793bd6".to_string();
    let registry_id =
        "0x952569689168ac41183bf1c9028034d2bf437ad817c7e6a21a155a5e95506fe7".to_string();
    let network = "testnet".to_string();
    let is_active = true;
    let created_at = DateTime::<Utc>::from_timestamp_millis(1_736_560_800_000)
        .unwrap()
        .into();

    let updated_at: DateTimeWithTimeZone = created_at;
    let version = 1;
    let vault_id = "0xccd1c8ef21955356210536278492c56f677d99ed9dbd2df5738672a76001c0e9".to_string();
    let create_cap_id =
        "0x84244e7ac30b25e71e99a1eb0b63dbb973bc0cfdb0b933a5a1ee270d4e4f6b97".to_string();
    let registry_initial_version = 309_765_686;
    let draw_cap_id =
        "0xbbc3b417fa1d9e8babca85bd0422b2e11645b6c652796d219c9923412fbc29ce".to_string();

    let _active_model = contracts::ActiveModel {
        package_id: Set(package_id.to_string()),
        registry_id: Set(registry_id.to_string()),
        network: Set(network.to_string()),
        is_active: Set(is_active),
        created_at: Set(created_at),
        updated_at: Set(updated_at),
        // id: Set(1),
        version: Set(version),
        vault_id: Set(vault_id),
        create_cap_id: Set(create_cap_id),
        registry_initial_version: Set(registry_initial_version),
        draw_cap_id: Set(draw_cap_id),
        ..Default::default()
    };

    // active_model.insert(&ctx.db).await.unwrap();

    // let res = contracts::Model::find_by_registry_id(&ctx.db, "0x952569689168ac41183bf1c9028034d2bf437ad817c7e6a21a155a5e95506fe7")
    //     .await
    //     .unwrap();
    let res = contracts::Model::find_latest(&ctx.db).await.unwrap();

    assert_eq!(res.registry_id, registry_id);
    assert_eq!(res.package_id, package_id);
    assert_eq!(res.network, network);
    assert!(res.is_active);
    // assert_eq!(res.id, 1);
    assert_eq!(res.version, 1);

    println!("{res:#?}");

    println!("welcome to playground. edit me at `examples/playground.rs`");

    Ok(())
}

// async fn create_contract(ctx: &AppContext) -> Result<()> {
//     let contract = contracts::ActiveModel {
//         package_id: Set("test".to_string()),
//         registry_id: Set("test".to_string()),
//         network: Set("test".to_string()),
//         is_active: Set(true),
//         created_at: Set(Local::now().into()),
//         updated_at: Set(Local::now().into()),
//         id: Set(1),
//         version: Set(1),
//     };

//     Ok(())
// }
