use serenity::{
    framework::Framework,
    model::channel::Message,
    client::Context
};
use threadpool::ThreadPool;
use std::{
    sync::mpsc::Sender,
    collections::HashMap
};
use tokio::prelude::*;
use rusoto_dynamodb::{
    DynamoDbClient,
    PutItemInput,
    AttributeValue,
    DynamoDb
};

pub struct ShillFramework {
    pub channel_tx: Sender<u64>,
    pub client: DynamoDbClient
}

impl Framework for ShillFramework {
    fn dispatch(&mut self, _: Context, _: Message, _: &ThreadPool) {
        let client = self.client.clone();
        tokio::spawn(
            db_put(String::from("test"),
                String::from("test_cat"),
                5,
                client));
    }
}

async fn db_put(name: String, category: String, count: u64,
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
}