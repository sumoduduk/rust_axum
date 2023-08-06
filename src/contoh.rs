use chrono::NaiveDateTime;
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use sqlx::{query, query_as, PgConnection};
use std::result::Result as StdResult;

#[derive(sqlx::FromRow, Debug)]
struct YourStruct {
    id: i32,
    image: String,
    time_created: NaiveDateTime,
    ipfs_image_url: String,
    category: Option<String>,
    updated_date: NaiveDateTime,
}

#[derive(Debug)]
enum CrudOperation {
    Create(YourStruct),
    Read(i32),
    Update(i32, YourStruct),
    Delete(i32),
}

impl CrudOperation {
    async fn execute(self, pool: &PgConnection) -> StdResult<YourStruct, sqlx::Error> {
        match self {
            CrudOperation::Create(item) => {
                let new_item = query!(
                    "INSERT INTO your_table_name (image, ipfs_image_url, category) VALUES ($1, $2, $3) RETURNING *",
                    item.image, item.ipfs_image_url, item.category
                )
                .fetch_one(pool)
                .await?;

                Ok(new_item)
            }
            CrudOperation::Read(id) => {
                let item = query_as!(
                    YourStruct,
                    "SELECT * FROM your_table_name WHERE id = $1",
                    id
                )
                .fetch_one(pool)
                .await?;

                Ok(item)
            }
            CrudOperation::Update(id, updated_item) => {
                let updated_item = query!(
                    "UPDATE your_table_name SET image = $1, ipfs_image_url = $2, category = $3 WHERE id = $4 RETURNING *",
                    updated_item.image, updated_item.ipfs_image_url, updated_item.category, id
                )
                .fetch_one(pool)
                .await?;

                Ok(updated_item)
            }
            CrudOperation::Delete(id) => {
                query!("DELETE FROM your_table_name WHERE id = $1", id)
                    .execute(pool)
                    .await?;

                Ok(YourStruct {
                    id: 0,
                    image: String::new(),
                    time_created: NaiveDateTime::from_timestamp(0, 0),
                    ipfs_image_url: String::new(),
                    category: None,
                    updated_date: NaiveDateTime::from_timestamp(0, 0),
                })
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_url = "postgres://username:password@localhost:5432/database_name";
    let pool = PgConnection::connect(&database_url).await?;

    // Example usage
    let create_result = CrudOperation::Create(YourStruct {
        id: 0,
        image: "example_image".to_string(),
        time_created: NaiveDateTime::from_timestamp(0, 0),
        ipfs_image_url: "example_ipfs".to_string(),
        category: Some("example_category".to_string()),
        updated_date: NaiveDateTime::from_timestamp(0, 0),
    })
    .execute(&pool)
    .await?;

    let read_result = CrudOperation::Read(create_result.id).execute(&pool).await?;

    let update_result = CrudOperation::Update(
        create_result.id,
        YourStruct {
            id: 0,
            image: "updated_image".to_string(),
            time_created: NaiveDateTime::from_timestamp(0, 0),
            ipfs_image_url: "updated_ipfs".to_string(),
            category: None,
            updated_date: NaiveDateTime::from_timestamp(0, 0),
        },
    )
    .execute(&pool)
    .await?;

    let delete_result = CrudOperation::Delete(update_result.id)
        .execute(&pool)
        .await?;

    println!("Create result: {:?}", create_result);
    println!("Read result: {:?}", read_result);
    println!("Update result: {:?}", update_result);
    println!("Delete result: {:?}", delete_result);

    Ok(())
}
