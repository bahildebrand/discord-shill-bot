use std::{
    env,
    default::Default
};
use std::collections::{
    HashMap,
    HashSet
};
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
    framework::{
        StandardFramework,
        standard::macros::group
    }
};
use log::{
    info,
    error,
    debug
};
use log4rs::init_file;
use rusoto_core::{
    Region
};
use rusoto_dynamodb::{
    DynamoDbClient,
    PutItemInput,
    AttributeValue,
    DynamoDb
};
use futures::executor::block_on;

mod commands;
use commands::COUNT_COMMAND;

mod shill_structs;
use shill_structs::{
    ShillCounter,
    ShillCategory,
    BotName,
    DataBase
};

struct Handler;

fn get_categories(ctx: &Context) -> HashSet<String> {
    let data = ctx.data.write();
    let categories = data.get::<ShillCategory>().unwrap();

    categories.clone()
}

fn check_for_bot_name(ctx: &Context, name: &String) -> bool {
    let data = ctx.data.write();
    let bot_name = data.get::<BotName>().unwrap();

    bot_name.contains(name)
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let lowercase_msg = msg.content.to_lowercase();

        let categories = get_categories(&ctx);

        // Ignore messages from bot and commands
        if lowercase_msg.contains("!shill") ||
            check_for_bot_name(&ctx, &msg.author.name)
        {
            return;
        }

        for category in categories.iter() {
            if lowercase_msg.contains(category) {

                let count = lowercase_msg.matches(category).count() as u64;
                inc_counter(&ctx, category, count);
            }
        }
    }

    fn ready(&self, ctx: Context, ready: Ready) {
        let mut data = ctx.data.write();

        {
            let category_set = data.get_mut::<ShillCategory>().unwrap();
            category_set.insert(String::from("ign"));
            category_set.insert(String::from("hyperx"));
        }

        {
            let botname_set = data.get_mut::<BotName>().unwrap();
            botname_set.insert(String::from(ready.user.name.clone()));
        }

        {
            let database_map = data.get_mut::<DataBase>().unwrap();

            let client = DynamoDbClient::new(Region::UsEast1);
            database_map.insert(String::from("DBClient"), client);
        }

        info!("{} is connected!", ready.user.name);
    }
}

fn inc_counter(ctx: &Context, name: &String, count: u64)
{
    let mut data = ctx.data.write();
    {
        let counter = data.get_mut::<ShillCounter>().unwrap();
        let entry = counter.entry(name.clone()).or_insert(0);
        *entry += count;

        debug!("{} shill count: {}", name, *entry);
    }

}

async fn db_put(name: &String, category: String, count: u64,
        client: DynamoDbClient) {
    let mut item_map = HashMap::new();

    item_map.insert(String::from("Name"), AttributeValue {
        s: Some(name.clone()),
        ..Default::default()
    });
    item_map.insert(String::from("Category"), AttributeValue {
        s: Some(category),
        ..Default::default()
    });
    item_map.insert(String::from("Count"), AttributeValue {
        n: Some(String::from(count.to_string())),
        ..Default::default()
    });
    let item = PutItemInput {
        table_name: String::from("ShillCount"),
        item: item_map,
        ..Default::default()
    };

    let ret = client.put_item(item).await;
    debug!("{:?}", ret);
}

#[group("shill")]
#[prefix = "shill"]
#[commands(count)]
struct Shill;

fn main() {
    init_file("log4rs.yml", Default::default()).unwrap();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler).expect("Err creating client");
    {
        let mut data = client.data.write();
        data.insert::<ShillCounter>(HashMap::default());
        data.insert::<ShillCategory>(HashSet::default());
        data.insert::<BotName>(HashSet::default());
        data.insert::<DataBase>(HashMap::default());
    }

    client.with_framework(
        StandardFramework::new()
            .group(&SHILL_GROUP)
            .configure(|c| {
                c.prefix("!")
            })
    );

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}