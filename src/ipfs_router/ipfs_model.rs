#![allow(dead_code)]

use serde::Serialize;
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
    width: i32,
    height: i32,
    prompt: Option<String>,
    hash_id: String,
}

pub enum Operation {
    Create {
        image: String,
        ipfs_image_url: String,
        category: Option<String>,
        width: i32,
        height: i32,
        prompt: Option<String>,
        hash_id: String,
    },
    Read,
    Fetch,
    Update(i32, Option<String>, Option<String>, Option<String>),
    Delete(i32),
}

#[derive(Debug)]
pub enum OperationResult {
    DataStruct(i32, String, String, Option<String>, String),
    UpdateStruct(ReturnJson),
    ArrStruct(ArrStructData),
    Deleted(i32),
    Error,
}

#[derive(Debug)]
pub enum ArrStructData {
    ReturnJsonEnum(Vec<ReturnJson>),
    SchemaEnum(Vec<SchemaIPFS>),
}

#[derive(Serialize, Debug)]
pub struct ReturnJson {
    pub id: i32,
    image: String,
    ipfs_image_url: String,
    category: Option<String>,
    created: Option<String>,
    updated_date: Option<String>,
}

use OperationResult::*;

fn datetime_to_string(datetime: Option<DateTime<Utc>>) -> Option<String> {
    datetime.map(|opt| opt.to_rfc3339())
}

impl Operation {
    pub async fn execute(&self, pool: &Pool<Postgres>) -> Result<OperationResult, sqlx::Error> {
        match self {
            Self::Create {
                image,
                ipfs_image_url,
                category,
                width,
                height,
                prompt,
                hash_id,
            } => {
                let inserted_data = Self::create_row(
                    pool,
                    image,
                    ipfs_image_url,
                    category,
                    width,
                    height,
                    prompt,
                    hash_id,
                )
                .await?;
                let (id, image, ipfs_image_url, category, hash_id) = inserted_data;
                Ok(DataStruct(id, image, ipfs_image_url, category, hash_id))
            }

            Self::Read => {
                let all_data = Self::read_all(pool).await?;
                Ok(ArrStruct(ArrStructData::SchemaEnum(all_data)))
            }

            Self::Update(id, image, ipfs_image_url, category) => {
                let updated_data = Self::update(pool, *id, image, ipfs_image_url, category).await?;

                Ok(UpdateStruct(updated_data))
            }

            Self::Delete(id) => {
                let id_affected = Self::delete_individual(pool, id).await?;
                dbg!(id_affected);
                Ok(Deleted(id_affected))
            }

            Self::Fetch => {
                let returned_arr = Self::read_all_ret(pool).await?;
                Ok(ArrStruct(ArrStructData::ReturnJsonEnum(returned_arr)))
            }
        }
    }
    async fn create_row(
        pool: &Pool<Postgres>,
        image: &str,
        ipfs_image_url: &str,
        category: &Option<String>,
        w: &i32,
        h: &i32,
        prompt: &Option<String>,
        hash_id: &str,
    ) -> Result<(i32, String, String, Option<String>, String), sqlx::Error> {
        let mut tx = pool.begin().await?;

        let inserted = sqlx::query!(
            r#"
                INSERT INTO ipfs_image (image, ipfs_image_url, category, width, height, prompt, hash_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id, image, ipfs_image_url, category, hash_id
            "#,
            image,
            ipfs_image_url,
            category.as_deref(),
            w,
            h,
            prompt.as_deref(),
            hash_id,
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;

        let (id, image, ipfs_image_url, category, hash_id) = (
            inserted.id,
            inserted.image,
            inserted.ipfs_image_url,
            inserted.category,
            inserted.hash_id,
        );

        println!("image {}", &image);

        Ok((id, image, ipfs_image_url, category, hash_id))
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

    async fn read_all_ret(pool: &Pool<Postgres>) -> Result<Vec<ReturnJson>, sqlx::Error> {
        let all_data: Vec<_> = sqlx::query!(
            r#"
            SELECT id, image, time_created, ipfs_image_url, category, updated_date
            FROM ipfs_image
            "#
        )
        .fetch_all(pool)
        .await?;
        dbg!(&all_data);

        let mapped_data = all_data
            .iter()
            .map(|elm| ReturnJson {
                id: elm.id,
                image: elm.image.to_owned(),
                ipfs_image_url: elm.ipfs_image_url.to_owned(),
                category: elm.category.to_owned(),
                created: datetime_to_string(elm.time_created.to_owned()),
                updated_date: datetime_to_string(elm.updated_date.to_owned()),
            })
            .collect();

        Ok(mapped_data)
    }

    async fn update(
        pool: &Pool<Postgres>,
        id: i32,
        image: &Option<String>,
        ipfs_image_url: &Option<String>,
        category: &Option<String>,
    ) -> Result<ReturnJson, sqlx::Error> {
        let mut tx = pool.begin().await?;

        let updated_res = sqlx::query!(
            r#"
                UPDATE ipfs_image
                SET 
                    image = COALESCE($1, image),
                    ipfs_image_url = COALESCE($2, ipfs_image_url),
                    category = COALESCE($3, category),
                    updated_date = NOW()
                WHERE id = $4
                RETURNING id, image, time_created, ipfs_image_url, category, updated_date
            "#,
            image.as_deref(),
            ipfs_image_url.as_deref(),
            category.as_deref(),
            id,
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;

        let updated_res_data = ReturnJson {
            id: updated_res.id,
            image: updated_res.image,
            ipfs_image_url: updated_res.ipfs_image_url,
            category: updated_res.category,
            created: datetime_to_string(updated_res.time_created),
            updated_date: datetime_to_string(updated_res.updated_date),
        };

        Ok(updated_res_data)
    }

    async fn delete_individual(pool: &Pool<Postgres>, id: &i32) -> Result<i32, sqlx::Error> {
        let mut tx = pool.begin().await?;
        let stmt = sqlx::query(
            r#"
            DELETE FROM ipfs_image
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&mut tx)
        .await?;
        tx.commit().await?;

        let res = stmt.rows_affected() as i32;
        dbg!(res);

        Ok(res)
    }
}

// #[cfg(test)]
// mod test {
//     use dotenv::dotenv;
//     use std::env;
//
//     fn parse_timestamp(s: &str) -> Option<DateTime<Utc>> {
//         match DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.fZ") {
//             Ok(dt) => Some(dt.into()),
//             Err(_) => None,
//         }
//     }
// }
//
// #[tokio::test]
// async fn it_can_success_fetch_all_data() {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     dbg!(&database_url);
//
//     let pool: Pool<Postgres> = sqlx::postgres::PgPoolOptions::new()
//         .max_connections(5)
//         .connect(&database_url)
//         .await
//         .unwrap();
//
//     let res = Operation::Read.execute(&pool).await.unwrap();
//
//     let dummy = vec![
//         SchemaIPFS {
//             id: 4,
//             image: "image1.jpg".to_string(),
//             time_created: parse_timestamp("2023-08-07T06:53:39.054261Z"),
//             ipfs_image_url: "https://ipfs.io/ipfs/Qm...".to_string(),
//             category: Some("category1".to_string()),
//             updated_date: parse_timestamp("2023-08-07T06:53:39.054261Z"),
//         },
//         SchemaIPFS {
//             id: 5,
//             image: "image2.jpg".to_string(),
//             time_created: parse_timestamp("2023-08-07T06:53:39.054261Z"),
//             ipfs_image_url: "https://ipfs.io/ipfs/Qm...".to_string(),
//             category: Some("category2".to_string()),
//             updated_date: parse_timestamp("2023-08-07T06:53:39.054261Z"),
//         },
//         SchemaIPFS {
//             id: 6,
//             image: "image3.jpg".to_string(),
//             time_created: parse_timestamp("2023-08-07T06:53:39.054261Z"),
//             ipfs_image_url: "https://ipfs.io/ipfs/Qm...".to_string(),
//             category: Some("category3".to_string()),
//             updated_date: parse_timestamp("2023-08-07T06:53:39.054261Z"),
//         },
//         SchemaIPFS {
//             id: 7,
//             image: "image4.jpg".to_string(),
//             time_created: parse_timestamp("2023-08-07T06:53:39.054261Z"),
//             ipfs_image_url: "https://ipfs.io/ipfs/Qm...".to_string(),
//             category: None,
//             updated_date: parse_timestamp("2023-08-07T06:53:39.054261Z"),
//         },
//     ];
//
//     if let OperationResult::ArrStruct(ArrStructData::SchemaEnum(data)) = res {
//         assert_eq!(dummy, data)
//     };
// }
