use axum::{
    http::StatusCode,
    routing::{delete, get, patch, post},
    Router,
};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{env, net::SocketAddr};

mod ipfs_router;

use ipfs_router::{
    begin_insert, create_data, delete_data, fetch_single, get_all_ipfs, get_all_pretty, test_query,
    update_data,
};

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
        .route("/get_pretty", get(get_all_pretty))
        .route("/create_data", post(create_data))
        .route("/update_data/:id", patch(update_data))
        .route("/delete_data/:id", delete(delete_data))
        .route("/fetch_single/:id", get(fetch_single))
        .route("/begin_insert", get(begin_insert))
        .route("/test_search_query", get(test_query))
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
