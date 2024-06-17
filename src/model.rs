use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

use crate::Platform;

// const MASK: &str = "%d/%m/%Y %H:%M";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub name: String,
    pub amount: f32,
    pub platform: Platform,
    pub price: f32,
    pub date: String,
}

impl Item {
    pub fn new(name: String, amount: f32, platform: Platform, price: f32, date: String) -> Self {
        Item {
            name,
            amount,
            platform,
            price,
            date,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rates {
    pub asset_id_base: String,
    pub rates: Vec<Rate>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rate {
    pub time: String,
    pub asset_id_quote: String,
    pub rate: f32,
}
