use axum::{extract::State, http::StatusCode, Json};

mod ipfs_model;

use ipfs_model::{Operation, OperationResult, ReturnJson};
use sqlx::{Pool, Postgres};

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
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<ReturnJson>>, (StatusCode, String)> {
}
