use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

mod ipfs_model;
mod seaart_resp;

use crate::internal_error;

use ipfs_model::{ArrStructData, Operation, OperationResult, ReturnJson};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{Pool, Postgres};

use ArrStructData::*;
use OperationResult::*;

use self::seaart_resp::{extract_obj, get_raw_value};

#[derive(Deserialize, Debug)]
pub struct CreatePayload {
    image: String,
    ipfs_image_url: String,
    category: Option<String>,
    width: i32,
    height: i32,
    prompt: Option<String>,
    hash_id: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdatePayload {
    image: Option<String>,
    ipfs_image_url: Option<String>,
    category: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct FormContact {
    name: String,
    email: String,
    phone: String,
    message: String,
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
    let res = Operation::Create {
        image: payload.image,
        ipfs_image_url: payload.ipfs_image_url,
        category: payload.category,
        width: payload.width,
        height: payload.height,
        prompt: payload.prompt,
        hash_id: payload.hash_id,
    }
    .execute(&pool)
    .await
    .map_err(internal_error);

    match res {
        Ok(resp) => match resp {
            DataStruct(id, image, ipfs_image_url, category, hash_id) => Ok(Json(json!({
                "id": id,
                "image": image,
                "ipfs_image_url": ipfs_image_url,
                "category" : category,
                "hash_id" : hash_id
            }))),
            _ => Err((StatusCode::NOT_FOUND, "Imposible".to_string())),
        },
        Err(_) => Err((StatusCode::NOT_FOUND, "Shit happen".to_string())),
    }
}

pub async fn contact_form(
    Json(payload): Json<FormContact>,
) -> Result<Json<Value>, (StatusCode, String)> {
    dbg!(&payload);

    Ok(Json(json!({
        "name": payload.name,
        "email": payload.email,
        "phone": payload.phone,
        "message": payload.message,
    })))
}

pub async fn update_data(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePayload>,
) -> Result<Json<ReturnJson>, (StatusCode, String)> {
    dbg!(&payload);
    let res = Operation::Update(id, payload.image, payload.ipfs_image_url, payload.category)
        .execute(&pool)
        .await
        .map_err(internal_error);

    match res {
        Ok(resp) => match resp {
            UpdateStruct(data) => Ok(Json(data)),
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

pub async fn begin_insert(
    Query(search_params): Query<HashMap<String, String>>,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let q_default = String::from("");
    let page_default = String::from("1");

    let mut tag_vec = Vec::new();

    let q = search_params.get("q").unwrap_or(&q_default);
    let category = search_params.get("category");
    let page = search_params.get("page").unwrap_or(&page_default);
    let tags = search_params.get("tags");

    let page: u16 = page.parse().unwrap_or(1);

    tag_vec.extend(tags);

    match get_raw_value(q, page, tag_vec).await {
        Ok(result) => {
            let vec_result = &result["data"]["items"];
            match vec_result.as_array() {
                Some(vec_val) => {
                    let mut count_inserted: u16 = 0;
                    let mut count_not_inserted: u16 = 0;
                    for item in vec_val {
                        let extracted_obj = extract_obj(item);

                        //insert to db;

                        let res = Operation::Create {
                            image: extracted_obj.0.to_string(),
                            ipfs_image_url: "NO_IPFS".to_owned(),
                            category: category.cloned(),
                            width: extracted_obj.3,
                            height: extracted_obj.4,
                            prompt: Some(extracted_obj.2.to_string()),
                            hash_id: extracted_obj.1.to_string(),
                        }
                        .execute(&pool)
                        .await
                        .map_err(internal_error);

                        match res {
                            Ok(_) => count_inserted += 1,
                            Err(_) => count_not_inserted += 1,
                        }
                    }
                    dbg!((count_not_inserted, count_inserted));
                    Ok(Json(json!({
                        "success": true,
                        "total_inserted": count_inserted,
                        "total_failure": count_not_inserted
                    })))
                }
                None => Err((StatusCode::NOT_FOUND, "Value not an array".to_owned())),
            }
        }
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}

pub async fn test_query(
    Query(search_params): Query<HashMap<String, String>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    dbg!(&search_params);

    let mut tag_vec: Vec<&String> = Vec::new();

    let empty_str = String::from("");
    let page_default = String::from("1");

    let q = search_params.get("q").unwrap_or(&empty_str);
    let category = search_params.get("category");
    let tags = search_params.get("tags");
    let page = search_params.get("page").unwrap_or(&page_default);

    let page = page.parse::<u16>().unwrap_or(1);

    tag_vec.extend(tags);

    Ok(Json(json!({
        "q": q,
        "category": category,
        "tags": tag_vec,
        "page": page
    })))
}
