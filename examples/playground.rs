use chrono::{DateTime, Utc};
use fairwin_be::app::App;
#[allow(unused_imports)]
use loco_rs::{cli::playground, prelude::*};

use fairwin_be::models::_entities::pools::{self, ActiveModel};

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    let ctx = playground::<App>().await?;

    let active_model: pools::ActiveModel = ActiveModel {
        pool_id: Set(Some("0x1ab10df973a6020f3cf158bbc095337229e6340b0a51191f4d6d1fa3e80e7095".to_string())),
        start_time: Set(Some(DateTime::<Utc>::from_timestamp_millis(1735122600000).unwrap().naive_utc())),
        end_time: Set(Some(DateTime::<Utc>::from_timestamp_millis(1735122600000).unwrap().naive_utc())),
        drawn_time: Set(Some(DateTime::<Utc>::from_timestamp_millis(1735122600000).unwrap().naive_utc())),
        created_time: Set(Some(DateTime::<Utc>::from_timestamp_millis(1735122600000).unwrap().naive_utc())),
        created_by: Set(Some("admin".to_string())),
        ..Default::default()
    };
    active_model.insert(&ctx.db).await.unwrap();

    // let res = articles::Entity::find().all(&ctx.db).await.unwrap();
    // println!("{:?}", res);
    let res = pools::Entity::find().all(&ctx.db).await.unwrap();
    println!("{:?}", res);
    println!("welcome to playground. edit me at `examples/playground.rs`");

    Ok(())
}
