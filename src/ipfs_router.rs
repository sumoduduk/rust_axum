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
use serde_json::{json, Value};
use sqlx::{Pool, Postgres};

use ArrStructData::*;
use OperationResult::*;

use self::seaart_resp::{extract_obj, get_raw_value};

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
