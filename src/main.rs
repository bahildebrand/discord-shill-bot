use std::{
    env,
};

use std::collections::{
    HashMap,
    HashSet
};

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct ShillCounter;

impl TypeMapKey for ShillCounter {
    type Value = HashMap<String, u64>;
}

struct ShillCategory;

impl TypeMapKey for ShillCategory {
    type Value = HashSet<String>;
}

struct Handler;

fn get_categories(ctx: &Context) -> HashSet<String> {
    let data = ctx.data.write();
    let categories = data.get::<ShillCategory>().unwrap();

    categories.clone()
}

impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    fn message(&self, ctx: Context, msg: Message) {
        let lowercase_msg = msg.content.to_lowercase();

        let categories = get_categories(&ctx);

        for category in categories.iter() {
            if lowercase_msg.contains(category) {
                // Sending a message can fail, due to a network error, an
                // authentication error, or lack of permissions to post in the
                // channel, so log to stdout when some error happens, with a
                // description of it.

                let count = lowercase_msg.matches(category).count() as u64;
                inc_counter(&ctx, category, count);

                if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    fn ready(&self, ctx: Context, ready: Ready) {
        let mut data = ctx.data.write();
        let set = data.get_mut::<ShillCategory>().unwrap();

        set.insert(String::from("ign"));
        set.insert(String::from("hyperx"));

        println!("{} is connected!", ready.user.name);
    }
}

fn inc_counter(ctx: &Context, name: &String, count: u64)
{
    let mut data = ctx.data.write();
    let counter = data.get_mut::<ShillCounter>().unwrap();
    let entry = counter.entry(name.clone()).or_insert(0);
    *entry += count;

    println!("{} shill count: {}", name, *entry);
}

fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    {
        let mut data = client.data.write();
        data.insert::<ShillCounter>(HashMap::default());
        data.insert::<ShillCategory>(HashSet::default());
    }

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}