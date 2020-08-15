use serenity::framework::standard::{
    macros::command,
    Args,
    CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use log::error;

use crate::shill_structs::DataBase;
use crate::db_manager::get_count;

#[command]
#[description = "Request the shill count for a given category"]
#[usage("!shill count <name> <category>")]
#[example("!shill count ign")]
pub async fn count(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() != 2 {
        let _ = msg.reply(&ctx, "Incorrect number of arguements").await;
        error!("Args {:?}", args);
    } else {
        let name = args.single::<String>().unwrap();
        let category = args.single::<String>().unwrap();

        let data = ctx.data.write().await;
        let db_client = data.get::<DataBase>().unwrap();

        let count = get_count(name.clone(), category.clone(),
            db_client.clone()).await;

        match count {
            Ok(c) => {
                let reply_str = format!("@{} has shilled for {} {} times",
                    name,
                    category,
                    c);
                let _ = msg.reply(&ctx, reply_str).await;
            },
            Err(e) => {
                let _ = msg.reply(&ctx, e).await;
            }
        }
    }

    Ok(())
}