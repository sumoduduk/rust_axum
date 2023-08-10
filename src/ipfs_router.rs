use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

mod ipfs_model;

use crate::internal_error;

use ipfs_model::{ArrStructData, Operation, OperationResult, ReturnJson};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::{Pool, Postgres};

use ArrStructData::*;
use OperationResult::*;

#[derive(Deserialize, Debug)]
pub struct CreatePayload {
    image: String,
    ipfs_image_url: String,
    category: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct UpdatePayload {
    image: Option<String>,
    ipfs_image_url: Option<String>,
    category: Option<String>,
}

pub async fn get_all_ipfs(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<ReturnJson>>, (StatusCode, String)> {
    let res = Operation::Fetch
        .execute(&pool)
        .await
        .map_err(internal_error);

    dbg!(&res);

    match res {
        Ok(response) => match response {
            ArrStruct(ReturnJsonEnum(data)) => Ok(Json(data)),
            _ => Err((StatusCode::NOT_FOUND, "Imposible".to_string())),
        },
        Err(_) => Err((StatusCode::NOT_FOUND, "Shit happen".to_string())),
    }
}

pub async fn get_all_pretty(
    State(pool): State<Pool<Postgres>>,
) -> Result<String, (StatusCode, String)> {
    let res = Operation::Fetch
        .execute(&pool)
        .await
        .map_err(internal_error);

    dbg!(&res);

    match res {
        Ok(response) => match response {
            ArrStruct(ReturnJsonEnum(data)) => {
                let json_value =
                    serde_json::to_string_pretty(&data).unwrap_or("Not Found".to_string());

                Ok(json_value)
            }
            _ => Err((StatusCode::NOT_FOUND, "Imposible".to_string())),
        },
        Err(_) => Err((StatusCode::NOT_FOUND, "Shit happen".to_string())),
    }
}

pub async fn create_data(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<CreatePayload>,
) -> Result<Json<Value>, (StatusCode, String)> {
    dbg!(&payload);
    let res = Operation::Create(payload.image, payload.ipfs_image_url, payload.category)
        .execute(&pool)
        .await
        .map_err(internal_error);

    match res {
        Ok(resp) => match resp {
            DataStruct(id, image, ipfs_image_url) => Ok(Json(json!({
                "id": id,
                "image": image,
                "ipfs_image_url": ipfs_image_url
            }))),
            _ => Err((StatusCode::NOT_FOUND, "Imposible".to_string())),
        },
        Err(_) => Err((StatusCode::NOT_FOUND, "Shit happen".to_string())),
    }
}

pub async fn update_data(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePayload>,
) -> Result<Json<Value>, (StatusCode, String)> {
    dbg!(&payload);
    let res = Operation::Update(id, payload.image, payload.ipfs_image_url, payload.category)
        .execute(&pool)
        .await
        .map_err(internal_error);

    match res {
        Ok(resp) => match resp {
            DataStruct(id, date, ipfs_image_url) => Ok(Json(json!({
                "id": id,
                "updated_at": date,
                "ipfs_image_url": ipfs_image_url
            }))),
            _ => Err((StatusCode::NOT_FOUND, "Imposible".to_string())),
        },
        Err(_) => Err((StatusCode::NOT_FOUND, "Shit happen".to_string())),
    }
}

pub async fn delete_data(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, (StatusCode, String)> {
    dbg!(id);
    let res = Operation::Delete(id)
        .execute(&pool)
        .await
        .map_err(internal_error);

    match res {
        Ok(resp) => match resp {
            Deleted(id) => Ok(Json(json!({
                "message" : format!("{id} successfully deleted")
            }))),
            _ => Err((StatusCode::NOT_FOUND, "Imposible".to_string())),
        },
        Err(err) => {
            dbg!(err);
            Err((StatusCode::NOT_FOUND, "Shit happen".to_string()))
        }
    }
}

pub async fn fetch_single(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<ReturnJson>, (StatusCode, String)> {
    dbg!(id);
    let res = Operation::Fetch
        .execute(&pool)
        .await
        .map_err(internal_error);

    match res {
        Ok(resp) => match resp {
            ArrStruct(ReturnJsonEnum(data)) => {
                dbg!(&data);
                let single_data: Option<ReturnJson> = data.into_iter().find(|s| s.id == id);

                match single_data {
                    Some(single) => Ok(Json(single)),
                    _ => Err((StatusCode::NOT_FOUND, "Id not found".to_string())),
                }
            }
            _ => Err((StatusCode::NOT_FOUND, "Imposible".to_string())),
        },
        Err(_) => Err((StatusCode::NOT_FOUND, "Shit happen".to_string())),
    }
}
