use rusoto_dynamodb::{
    DynamoDbClient,
    PutItemInput,
    AttributeValue,
    DynamoDb
};
use std::collections::HashMap;

pub async fn put_category_update(name: String, category: String, count: u64,
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