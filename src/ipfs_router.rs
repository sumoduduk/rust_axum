use axum::{extract::State, http::StatusCode, Json};

mod ipfs_model;

use ipfs_model::{Operation, OperationResult, ReturnJson, SchemaIPFS};
use sqlx::{Pool, Postgres};

struct CreatePayload {
    image: String,
    ipfs_image_url: String,
    category: Option<String>,
}

pub async fn get_all_ipfs(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<ReturnJson>>, (StatusCode, String)> {
    let res: OperationResult = Operation::Fetch.execute(&pool).await.unwrap();

    if let OperationResult::ArrStruct(ipfs_model::ArrStructData::ReturnJsonEnum(data)) = res {
        Ok(Json(data))
    } else {
        Err((StatusCode::NOT_FOUND, "Shit Happen".to_string()))
    }
}

pub async fn create_data(
    Json(payload): Json<CreatePayload>,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<SchemaIPFS>>, (StatusCode, String)> {
    let res = Operation::Create(payload.image, payload.ipfs_image_url, payload.category)
        .execute(&pool)
        .await
        .unwrap();

    if let OperationResult::ArrStruct(ipfs_model::ArrStructData::SchemaEnum(data)) = res {
        Ok(Json(data))
    } else {
        Err((StatusCode::NOT_FOUND, "Shit Happen".to_string()))
    }
}
