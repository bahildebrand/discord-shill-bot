use rusoto_dynamodb::{
    DynamoDbClient,
    UpdateItemInput,
    AttributeValue,
    DynamoDb,
    GetItemInput};
use std::collections::HashMap;
use log::{error, info};

pub async fn update_category_count(name: String, category: String, count: u64,
    client: DynamoDbClient, table_name: String) {
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
        table_name: table_name,
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

pub async fn get_count(name: String, category: String, client: DynamoDbClient,
            table_name: String)
        -> Result<u64, &'static str> {
    let mut key_map = HashMap::new();

    key_map.insert(String::from("Name"), AttributeValue {
        s: Some(name.clone()),
        ..Default::default()
    });
    key_map.insert(String::from("Category"), AttributeValue {
        s: Some(category),
        ..Default::default()
    });

    let item = GetItemInput {
        table_name: table_name,
        key: key_map,
        ..Default::default()
    };

    let ret = client.get_item(item).await;

    match ret {
        Ok(output) => {
            let result_map = output.item.unwrap();
            let attr = result_map.get("Count").unwrap();
            Ok(attr.n.as_ref().unwrap().parse::<u64>().unwrap())
        },
        Err(e) => {
            error!("Count get failed {}", e);
            Err("Can't reach database")
        }
    }
}