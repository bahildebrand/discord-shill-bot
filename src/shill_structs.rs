use std::collections::{
    HashMap,
    HashSet
};
use serenity::prelude::TypeMapKey;

pub struct ShillCounter;

impl TypeMapKey for ShillCounter {
    type Value = HashMap<String, u64>;
}

pub struct ShillCategory;

impl TypeMapKey for ShillCategory {
    type Value = HashSet<String>;
}

pub struct BotName;

impl TypeMapKey for BotName {
    type Value = HashSet<String>;
}