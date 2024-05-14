use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Foo {
    pub name: String,
    pub item: u64,
}

impl Foo {
    pub fn new(item: u64) -> Self {
        Self {
            name: item.to_string(),
            item,
        }
    }
}

impl Config {
    pub fn new(size: u64) -> Self {
        let mut data = vec![];
        for _ in 0..size {
            data.push(u8::MAX);
        }

        Self { data }
    }
}

pub const ITEMS: Map<u64, Foo> = Map::new("items");

pub const NUMBERS: Map<u64, u128> = Map::new("numbers");

pub const ADMIN: Item<Addr> = Item::new("admin");
pub const CONFIG: Item<Config> = Item::new("config");

pub const TOKEN: Item<String> = Item::new("registry");
