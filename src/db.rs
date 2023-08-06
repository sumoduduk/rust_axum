use sqlx::{postgres::PgRow, FromRow, PgConnection };

#[derive(FromRow, Debug)]
struct YourStruct {
    id: i32,
    image: String,
    time_created: NaiveDateTime,
    ipfs_image_url: String,
    category: Option<String>,
    updated_date: NaiveDateTime,
}

#[derive(FromRow, Debug)]
enum Operation {
    Create,
    Read,
    Update,
    Delete,
}

impl Operation {

    fn open_pool() {
        let 
    }
    
    fn create_row(){

    }
}
