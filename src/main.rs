mod commands;
mod db_manager;
mod shill_structs;
mod message_parser;

use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;
use std::{collections::HashSet, env};
use serenity::{
    async_trait,
    framework::standard::{
        CommandResult, DispatchError, StandardFramework,
        macros::{group, hook}
    },
    http::Http,
    model::{
        channel::Message,
        gateway::Ready
    },
};

use serenity::prelude::*;
use log::{
    info,
    error
};
use log4rs::init_file;

use message_parser::{MessageParser, ParseResult};
use commands::{COUNT_COMMAND, LEADERBOARD_COMMAND};


use shill_structs::{
    ShillCategory,
    BotName,
    DataBase,
    TableName
};

use db_manager::update_category_count;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let mut data = ctx.data.write().await;
        {
            let bot_name = data.get_mut::<BotName>().unwrap();

            *bot_name = ready.user.name.clone();
        }

        {
            let category_set = data.get_mut::<ShillCategory>().unwrap();
            category_set.insert(String::from("ign"));
            category_set.insert(String::from("hyperx"));
        }

        info!("{} is connected!", ready.user.name);
    }
}

#[group]
#[prefix = "shill"]
#[commands(count, leaderboard)]
struct Shill;

#[hook]
async fn before(_: &Context, msg: &Message, command_name: &str) -> bool {
    info!("Got command '{}' by user '{}'", command_name, msg.author.name);

    true // if `before` returns false, command processing doesn't happen.
}

#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => info!("Processed command '{}'", command_name),
        Err(why) => info!("Command '{}' returned error {:?}", command_name, why),
    }
}

#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    info!("Could not find command named '{}'", unknown_command_name);
}

async fn inc_counter(ctx: &Context, name: &String, result: &ParseResult)
{
    let data = ctx.data.write().await;

    let db_client = data.get::<DataBase>().unwrap();
    let table_name = data.get::<TableName>().unwrap();
    update_category_count(name.clone(), result.category.clone(), result.count,
        db_client.clone(), table_name.clone()).await;
}

async fn get_categories(ctx: &Context) -> HashSet<String> {
    let mut data = ctx.data.write().await;
    let categories = data.get_mut::<ShillCategory>().unwrap();

    categories.clone()
}

async fn check_for_bot_name(ctx: &Context, name: &String) -> bool {
    let data = ctx.data.write().await;
    let bot_name = data.get::<BotName>().unwrap();

    bot_name.contains(name)
}

#[hook]
async fn normal_message(ctx: &Context, msg: &Message) {
    // Ignore messages from bot and commands
    if check_for_bot_name(&ctx, &msg.author.name).await
    {
        return;
    }

    let youtube_token = env::var("YOUTUBE_TOKEN").expect(
        "Youtube token env variable not found.",
    );
    let parser = MessageParser::new(String::from(youtube_token));

    let categories = get_categories(&ctx).await;
    let parse_results = parser.parse(&msg.content, categories).await;

    match parse_results {
        Some(results) => {
            for result in results {
                inc_counter(&ctx, &msg.author.name, &result).await;
            }
        },
        None => info!("No matches found in message")
    }
}

#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    if let DispatchError::Ratelimited(seconds) = error {
        let _ = msg
            .channel_id
            .say(&ctx.http, &format!("Try this again in {} seconds.", seconds))
            .await;
    }
}

#[tokio::main]
async fn main() {
    init_file("log4rs.yml", Default::default()).unwrap();

    // Configure the client with your Discord bot token in the environment.
    let discord_token = env::var("DISCORD_TOKEN").expect(
        "Discord bot token env variable not found.",
    );
    let table_name = env::var("DB_TABLE_NAME").expect(
        "DynamoDB table name env variable not found.",
    );

    let http = Http::new_with_token(&discord_token);

    // We will fetch your bot's owners and id
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
                   .with_whitespace(true)
                   .on_mention(Some(bot_id))
                   .prefix("~")
                   .owners(owners))
        .before(before)
        .after(after)
        .unrecognised_command(unknown_command)
        .normal_message(normal_message)
        .on_dispatch_error(dispatch_error)
        .group(&SHILL_GROUP);

    let mut client = Client::new(&discord_token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShillCategory>(HashSet::default());
        data.insert::<BotName>(String::default());
        data.insert::<DataBase>(DynamoDbClient::new(Region::UsEast1));
        data.insert::<TableName>(table_name.clone());
    }

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}