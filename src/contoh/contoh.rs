use axum::{
    extract::{Extension, Path},
    handler::{get, post},
    http::StatusCode,
    response::IntoResponse,
    routing::BoxRoute,
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{convert::Infallible, env, net::SocketAddr};

#[derive(Debug, Serialize)]
struct YourTableName {
    id: i32,
    image: String,
    time_created: chrono::NaiveDateTime,
    ipfs_image_url: String,
    category: Option<String>,
    updated_date: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
struct CreateInput {
    image: String,
    ipfs_image_url: String,
    category: Option<String>,
}

#[derive(Deserialize)]
struct UpdateInput {
    category: Option<String>,
}

enum Operation {
    Create(String, String, Option<String>),
    Read,
    Update(i32, Option<String>),
    Delete(i32),
}

impl Operation {
    async fn execute(&self, pool: &Pool<Postgres>) -> Result<impl IntoResponse, Infallible> {
        match self {
            Self::Create(image, ipfs_image_url, category) => {
                let your_table_name = Self::create(pool, image, ipfs_image_url, category).await?;
                Ok((StatusCode::CREATED, your_table_name))
            }
            Self::Read => {
                let your_table_names = Self::read(pool).await?;
                Ok((StatusCode::OK, your_table_names))
            }
            Self::Update(id, category) => {
                let your_table_name = Self::update(pool, *id, category).await?;
                Ok((StatusCode::OK, your_table_name))
            }
            Self::Delete(id) => {
                let rows_affected = Self::delete(pool, *id).await?;
                if rows_affected == 0 {
                    Ok((StatusCode::NOT_FOUND, "Not found".to_string()))
                } else {
                    Ok((StatusCode::NO_CONTENT, ()))
                }
            }
        }
    }

    async fn create(
        pool: &Pool<Postgres>,
        image: &str,
        ipfs_image_url: &str,
        category: &Option<String>,
    ) -> Result<YourTableName, sqlx::Error> {
        let mut tx = pool.begin().await?;
        let your_table_name = sqlx::query_as!(
            YourTableName,
            r#"
                INSERT INTO ipfs_image (image, ipfs_image_url, category)
                VALUES ($1, $2, $3)
                RETURNING *
            "#,
            image,
            ipfs_image_url,
            category
        )
        .fetch_one(&mut tx)
        .await?;
        tx.commit().await?;

        Ok(your_table_name)
    }

    async fn read(pool: &Pool<Postgres>) -> Result<Vec<YourTableName>, sqlx::Error> {
        let your_table_names = sqlx::query_as!(
            YourTableName,
            r#"
                SELECT * FROM your_table_name
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(your_table_names)
    }

    async fn update(
        pool: &Pool<Postgres>,
        id: i32,
        category: &Option<String>,
    ) -> Result<YourTableName, sqlx::Error> {
        let mut tx = pool.begin().await?;
        let your_table_name = sqlx::query_as!(
            YourTableName,
            r#"
                UPDATE your_table_name
                SET category = $1
                WHERE id = $2
                RETURNING *
            "#,
            category,
            id
        )
        .fetch_one(&mut tx)
        .await?;
        tx.commit().await?;

        Ok(your_table_name)
    }

    async fn delete(pool: &Pool<Postgres>, id: i32) -> Result<u64, sqlx::Error> {
        let rows_affected = sqlx::query!(
            r#"
                DELETE FROM your_table_name
                WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?
        .rows_affected();

        Ok(rows_affected)
    }
}

async fn create(
    Extension(pool): Extension<Pool<Postgres>>,
    input: serde_json::Value,
) -> impl IntoResponse {
    let input = match serde_json::from_value::<CreateInput>(input) {
        Ok(input) => input,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid input".to_string()),
    };

    Operation::Create(input.image, input.ipfs_image_url, input.category)
        .execute(pool)
        .await
}

async fn read(Extension(pool): Extension<Pool<Postgres>>) -> impl IntoResponse {
    Operation::Read.execute(pool).await
}

async fn update(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
    input: serde_json::Value,
) -> impl IntoResponse {
    let input = match serde_json::from_value::<UpdateInput>(input) {
        Ok(input) => input,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid input".to_string()),
    };

    Operation::Update(*id, input.category).execute(pool).await
}

async fn delete(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    Operation::Delete(*id).execute(pool).await
}

fn app(pool: Pool<Postgres>) -> Router<BoxRoute> {
    Router::new()
        .route("/your_table_name", post(create))
        .route("/your_table_name", get(read))
        .route("/your_table_name/:id", post(update))
        .route("/your_table_name/:id", get(delete))
        .layer(axum::AddExtensionLayer::new(pool))
        .boxed()
}

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let app = app(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
