use db::{Operation, OperationResult};
use dotenv::dotenv;
use sqlx::{
    postgres::PgPoolOptions,
    types::chrono::{DateTime, Utc},
    Pool, Postgres,
};
use std::env;

mod db;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let res = Operation::Read.execute(&pool).await.unwrap();

    if let OperationResult::ArrStruct(data) = res {
        println!("{:?}", data);
    };
}

fn parse_timestamp(timestamp: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(timestamp)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}
