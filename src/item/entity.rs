use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub _id: String,
    pub name: String,
    pub description: String,
    pub damage: i32,
    pub level_required: i32,
    pub price: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemBson {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    pub damage: i32,
    pub level_required: i32,
    pub price: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertItemReq {
    pub name: String,
    pub description: String,
    pub damage: i32,
    pub level_required: i32,
    pub price: i32
}

impl Item {
    pub fn new() -> Self {
        Item {
            _id: String::from(""),
            name: String::from(""),
            description: String::from(""),
            damage: 0,
            level_required: 0,
            price: 0
        }
    }
}