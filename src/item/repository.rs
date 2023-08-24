use bson::{oid::ObjectId, from_document};
use mongodb::{bson::{doc, Document}, Cursor};

use crate::config::database::dbconnect;

use super::entity::{InsertItemReq, Item, ItemBson};
use std::result::Result;
use tracing::log::info;

pub async fn find_items() -> Vec<Item> {
    let db = match dbconnect().await {
        Ok(r) => r,
        Err(e) => panic!("Error: Database connection failed: {:?}", e)
    };

    let col = db.collection::<Document>("items");

    let cursor_result = col.find(doc! {}, None).await;
    let mut cursor: Cursor<Document> = match cursor_result {
        Ok(r) => r,
        Err(e) => {
            info!("Error: find items failed: {:?}", e);
            return Vec::new()
        }
    };

    let mut items: Vec<Item> = Vec::new();
    while let Ok(next) = cursor.advance().await {
        if !next {
            break
        }

        let item_doc = match cursor.deserialize_current().ok() {
            Some(doc) => doc,
            None => break
        };

        let item: ItemBson = match from_document(item_doc).map_err(|e| format!("Error: deserialize object failed: {:?}", e)) {
            Ok(i) => i,
            Err(e) => {
                info!("{:?}", e);
                return Vec::new()
            }
        };

        items.push(Item {
            _id: item._id.to_hex(),
            name: item.name,
            description: item.description,
            damage: item.damage,
            level_required: item.level_required,
            price:item.price
        })
    }

    items
}

pub async fn insert_one_item(req: InsertItemReq) -> Result<ObjectId, String> {
    let db = match dbconnect().await {
        Ok(r) => r,
        Err(e) => panic!("Error: Database connection failed: {:?}", e)
    };

    let col = db.collection::<Document>("items");

    let result = col.insert_one(doc! {
        "name": req.name,
        "description": req.description,
        "damage": req.damage,
        "level_required": req.level_required,
        "price": req.price
    }, None).await;

    let inserted_id_bson = match result {
        Ok(r) => r.inserted_id,
        Err(e) => {
            info!("Error: Insert one item failed: {:?}", e);
            return Err(format!("Error: Insert one item failed"))
        }
    };

    match inserted_id_bson.as_object_id() {
        Some(r) => Ok(r),
        None => {
            info!("Error: Parsing objectId failed");
            Err(format!("Error: Insert one item failed"))
        }
    }
}

pub async fn find_one_item(item_id: ObjectId) -> Result<Item, String> {
    let db = match dbconnect().await {
        Ok(r) => r,
        Err(e) => panic!("Error: Database connection failed: {:?}", e)
    };

    let col = db.collection::<Document>("items");

    let cursor = col.find_one(doc! {"_id": item_id}, None).await;
    let doc = match cursor {
        Ok(r) => r,
        Err(e) => {
            info!("Error: Find one item failed: {:?}", e);
            return Err(format!("Error: Find one item failed"))
        }
    };

    let item: ItemBson = match doc {
        Some(r) => match from_document(r).map_err(|e| format!("Error: deserialize object failed: {:?}", e)) {
            Ok(i) => i,
            Err(e) => {
                info!("{:?}", e);
                return Err(format!("Error: Deserialize object failed"))
            }
        }
        None => {
            info!("Error: Item not found");
            return Err(format!("Error: Item not found"))
        }
    };

    Ok(Item {
        _id: item._id.to_hex(),
        name: item.name,
        description: item.description,
        damage: item.damage,
        level_required: item.level_required,
        price: item.price
    })
}