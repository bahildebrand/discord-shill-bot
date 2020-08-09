use serenity::framework::standard::{
    macros::command,
    Args,
    CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::shill_structs::{
    ShillCounter
};

#[command]
#[description = "Request the shill count for a given category"]
#[usage("!shill count <category>")]
#[example("!shill count ign")]
pub fn count(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() != 1 {
        let _ = msg.reply(&ctx, "Incorrect number of arguements");
    } else {
        let category = args.single::<String>().unwrap();

        let mut data = ctx.data.write();
        let counter = data.get_mut::<ShillCounter>().unwrap();
        let entry = counter.get(&category);

        let reply_str = format!("{} count {}", category, entry.unwrap());
        let _ = msg.reply(&ctx, reply_str);
    }

    Ok(())
}