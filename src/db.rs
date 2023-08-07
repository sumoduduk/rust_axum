#![allow(dead_code)]

use std::env;

use chrono::NaiveDateTime;
use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    FromRow, PgConnection, Pool, Postgres,
};

#[derive(FromRow)]
struct SchemaIPFS {
    id: i32,
    image: String,
    time_created: NaiveDateTime,
    ipfs_image_url: String,
    category: Option<String>,
    updated_date: Option<NaiveDateTime>,
}

enum Operation {
    Create,
    Read,
    Update,
    Delete,
}

impl Operation {
    async fn create_row(
        pool: &Pool<Postgres>,
        image: &str,
        ipfs_image_url: &str,
        category: Option<String>,
    ) -> Result<SchemaIPFS, sqlx::Error> {
        let mut tx = pool.begin().await?;

        let inserted = sqlx::query_as!(
            SchemaIPFS,
            r#"
                INSERT INTO ipfs_image (image, ipfs_image_url, category)
                VALUES ($1, $2, $3)
                RETURNING *
            "#,
            image,
            ipfs_image_url,
            category,
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(inserted)
    }
}
