use axum::{response::{Json, IntoResponse}, extract::Path, http::StatusCode};
use bson::oid::ObjectId;
use serde_json::json;

use super::{entity::InsertItemReq, usecase};

pub async fn find_items() -> impl IntoResponse {
    (StatusCode::OK, Json(usecase::find_items().await).into_response())
}

pub async fn insert_one_item(Json(req): Json<InsertItemReq>) -> impl IntoResponse {
    let item_id = match usecase::insert_one_item(req).await {
        Ok(id) => id,
        Err(e) => return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": e,
            })).into_response()
        )
    };

    match usecase::find_one_item(item_id).await {
        Ok(r) => (
            StatusCode::CREATED,
            Json(r).into_response()
        ),
        Err(_) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "Error: Insert one item failed",
            })).into_response()
        )
    }
}

pub async fn find_one_item(Path(item_id): Path<String>) -> impl IntoResponse {
    let item_object_id = match ObjectId::parse_str(item_id) {
        Ok(id) => id,
        Err(_) => return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "Error: Parsing objectId failed"
            })).into_response()
        )
    };

    match usecase::find_one_item(item_object_id).await {
        Ok(r) => (
            StatusCode::OK,
            Json(r).into_response()
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": e,
            })).into_response()
        )
    }
}