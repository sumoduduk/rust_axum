#![allow(dead_code)]

use sqlx::{
    types::chrono::{DateTime, Utc},
    FromRow, Pool, Postgres,
};

#[derive(FromRow, Debug, PartialEq)]
pub struct SchemaIPFS {
    id: i32,
    image: String,
    time_created: Option<DateTime<Utc>>,
    ipfs_image_url: String,
    category: Option<String>,
    updated_date: Option<DateTime<Utc>>,
}

pub enum Operation {
    Create(String, String, Option<String>),
    Read,
    Update(i32, Option<String>),
    Delete(i32),
}

#[derive(Debug)]
pub enum OperationResult {
    DataStruct(SchemaIPFS),
    ArrStruct(Vec<SchemaIPFS>),
    Deleted,
    Error,
}

use OperationResult::*;

use crate::parse_timestamp;

impl Operation {
    pub async fn execute(&self, pool: &Pool<Postgres>) -> Result<OperationResult, sqlx::Error> {
        match self {
            Self::Create(image, ipfs_image_url, category) => {
                let inserted_data = Self::create_row(pool, image, ipfs_image_url, category).await?;
                Ok(DataStruct(inserted_data))
            }

            Self::Read => {
                let all_data = Self::read_all(pool).await?;
                Ok(ArrStruct(all_data))
            }

            Self::Update(id, category) => {
                let updated_data = Self::update(pool, *id, category).await?;
                Ok(DataStruct(updated_data))
            }

            Self::Delete(id) => {
                Self::delete_individual(pool, *id).await?;
                Ok(Deleted)
            }
        }
    }
    async fn create_row(
        pool: &Pool<Postgres>,
        image: &str,
        ipfs_image_url: &str,
        category: &Option<String>,
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
            category.as_deref(),
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(inserted)
    }

    async fn read_all(pool: &Pool<Postgres>) -> Result<Vec<SchemaIPFS>, sqlx::Error> {
        let all_data = sqlx::query_as!(
            SchemaIPFS,
            r#"
            Select * from ipfs_image
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(all_data)
    }

    async fn update(
        pool: &Pool<Postgres>,
        id: i32,
        category: &Option<String>,
    ) -> Result<SchemaIPFS, sqlx::Error> {
        let mut tx = pool.begin().await?;

        let updated_data = sqlx::query_as!(
            SchemaIPFS,
            r#"
                UPDATE ipfs_image
                SET category = $1
                WHERE id = $2
                RETURNING *
            "#,
            category.as_deref(),
            id
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(updated_data)
    }

    async fn delete_individual(pool: &Pool<Postgres>, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            Delete from ipfs_image
            Where id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

#[tokio::test]
async fn it_can_success_fetch_all_data() {
    use dotenv::dotenv;
    use std::env;

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    dbg!(&database_url);

    let pool: Pool<Postgres> = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let res = Operation::Read.execute(&pool).await.unwrap();

    let dummy = vec![
        SchemaIPFS {
            id: 4,
            image: "image1.jpg".to_string(),
            time_created: parse_timestamp("2023-08-07T06:53:39.054261Z"),
            ipfs_image_url: "https://ipfs.io/ipfs/Qm...".to_string(),
            category: Some("category1".to_string()),
            updated_date: parse_timestamp("2023-08-07T06:53:39.054261Z"),
        },
        SchemaIPFS {
            id: 5,
            image: "image2.jpg".to_string(),
            time_created: parse_timestamp("2023-08-07T06:53:39.054261Z"),
            ipfs_image_url: "https://ipfs.io/ipfs/Qm...".to_string(),
            category: Some("category2".to_string()),
            updated_date: parse_timestamp("2023-08-07T06:53:39.054261Z"),
        },
        SchemaIPFS {
            id: 6,
            image: "image3.jpg".to_string(),
            time_created: parse_timestamp("2023-08-07T06:53:39.054261Z"),
            ipfs_image_url: "https://ipfs.io/ipfs/Qm...".to_string(),
            category: Some("category3".to_string()),
            updated_date: parse_timestamp("2023-08-07T06:53:39.054261Z"),
        },
        SchemaIPFS {
            id: 7,
            image: "image4.jpg".to_string(),
            time_created: parse_timestamp("2023-08-07T06:53:39.054261Z"),
            ipfs_image_url: "https://ipfs.io/ipfs/Qm...".to_string(),
            category: None,
            updated_date: parse_timestamp("2023-08-07T06:53:39.054261Z"),
        },
    ];

    if let OperationResult::ArrStruct(data) = res {
        assert_eq!(dummy, data)
    };
}