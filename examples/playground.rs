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

    let active_model = contracts::ActiveModel {
        package_id: Set("test".to_string()),
        registry_id: Set("test".to_string()),
        network: Set("test".to_string()),
        is_active: Set(true),
        created_at: Set(DateTime::<Utc>::from_timestamp_millis(1735122600000)
            .unwrap()
            .into()),
        updated_at: Set(DateTime::<Utc>::from_timestamp_millis(1735122600000)
            .unwrap()
            .into()),
        id: Set(1),
        version: Set(1),
    };

    active_model.insert(&ctx.db).await.unwrap();

    let res = contracts::Model::find_by_registry_id(&ctx.db, "test")
        .await
        .unwrap();

    assert_eq!(res.registry_id, "test");
    assert_eq!(res.package_id, "test");
    assert_eq!(res.network, "test");
    assert_eq!(res.is_active, true);
    assert_eq!(res.id, 1);
    assert_eq!(res.version, 1);

    println!("{:?}", res);

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
