use std::collections::{
    HashSet
};

use serenity::prelude::TypeMapKey;
use rusoto_dynamodb::DynamoDbClient;

pub struct ShillCategory;

impl TypeMapKey for ShillCategory {
    type Value = HashSet<String>;
}

pub struct BotName;

impl TypeMapKey for BotName {
    type Value = String;
}

pub struct DataBase;

impl TypeMapKey for DataBase {
    type Value = DynamoDbClient;
}