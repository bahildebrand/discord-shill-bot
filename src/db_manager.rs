use rusoto_dynamodb::{DynamoDbClient, UpdateItemInput, AttributeValue, DynamoDb};
use std::collections::HashMap;
use log::{error, info};

pub async fn put_category_update(name: String, category: String, count: u64,
    client: DynamoDbClient) {
    let mut key_map = HashMap::new();
    let mut expr_val_map = HashMap::new();
    let mut expr_name_map = HashMap::new();

    key_map.insert(String::from("Name"), AttributeValue {
        s: Some(name.clone()),
        ..Default::default()
    });
    key_map.insert(String::from("Category"), AttributeValue {
        s: Some(category),
        ..Default::default()
    });

    expr_val_map.insert(String::from(":v"), AttributeValue {
        n: Some(count.to_string()),
        ..Default::default()
    });
    expr_name_map.insert(String::from("#C"), String::from("Count"));

    let item = UpdateItemInput {
        table_name: String::from("ShillCount"),
        update_expression: Some(String::from("SET #C = #C + :v")),
        key: key_map,
        return_values: Some(String::from("ALL_NEW")),
        expression_attribute_values: Some(expr_val_map),
        expression_attribute_names: Some(expr_name_map),
        ..Default::default()
    };

    let ret = client.update_item(item).await;

    match ret {
        Ok(new_vals) => info!("New vals {:?}", new_vals),
        Err(e) => error!("Category update failed {}", e)
    }
}