use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

#[derive(Debug)]
struct YourTableName {
    id: i32,
    image: String,
    time_created: chrono::NaiveDateTime,
    ipfs_image_url: String,
    category: Option<String>,
    updated_date: chrono::NaiveDateTime,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Create
    let mut tx = pool.begin().await?;
    let your_table_name = sqlx::query_as!(
        YourTableName,
        r#"
            INSERT INTO your_table_name (image, ipfs_image_url, category)
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
        "image.jpg",
        "https://ipfs.io/ipfs/Qm...",
        Some("category")
    )
    .fetch_one(&mut tx)
    .await?;
    tx.commit().await?;

    println!("Created: {:?}", your_table_name);

    // Read
    let your_table_names = sqlx::query_as!(
        YourTableName,
        r#"
            SELECT * FROM your_table_name
            WHERE category = $1
        "#,
        Some("category")
    )
    .fetch_all(&pool)
    .await?;

    println!("Read: {:?}", your_table_names);

    // Update
    let mut tx = pool.begin().await?;
    let your_table_name = sqlx::query_as!(
        YourTableName,
        r#"
            UPDATE your_table_name
            SET category = $1
            WHERE id = $2
            RETURNING *
        "#,
        Some("new_category"),
        your_table_name.id
    )
    .fetch_one(&mut tx)
    .await?;
    tx.commit().await?;

    println!("Updated: {:?}", your_table_name);

    // Delete
    let rows_affected = sqlx::query!(
        r#"
            DELETE FROM your_table_name
            WHERE id = $1
        "#,
        your_table_name.id
    )
    .execute(&pool)
    .await?
    .rows_affected();

    println!("Deleted {} row(s)", rows_affected);

    Ok(())
}
