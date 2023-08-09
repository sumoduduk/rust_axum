use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{env, net::SocketAddr};

mod ipfs_router;

use ipfs_router::{create_data, delete_data, get_all_ipfs, update_data};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(home))
        .route("/get_all", get(get_all_ipfs))
        .route("/create_data", post(create_data))
        .route("/update_data", post(update_data))
        .route("/delete_data", post(delete_data))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home() -> String {
    "Hello World".to_string()
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
