use axum::{
    http::StatusCode, response::IntoResponse, Json,
};
use bson::oid::ObjectId;
use serde_json::json;

use super::{entity::{InsertItemReq, Item}, repository};

pub async fn insert_one_item(req: InsertItemReq) -> Result<ObjectId, String> {
    repository::insert_one_item(req).await
}

pub async fn find_one_item(item_id: ObjectId) -> Result<Item, String> {
    repository::find_one_item(item_id).await
}