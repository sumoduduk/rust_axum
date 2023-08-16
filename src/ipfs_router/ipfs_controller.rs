use reqwest::Client;
use serde_json::{json, Value};

const USER_AGENT :&str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36";
const STORAGE_URI: &str = "https://api.nft.storage/";

pub async fn get_resp_data(query_search: &str) -> Result<Value, reqwest::Error> {
    let keyword = query_search.replace("+", " ");
    dbg!(&keyword);
    let url = "https://www.seaart.ai/api/v1/artwork/list";

    let payload = json!({
        "keyword": keyword,
        "order_by": "hot",
        "page": 1,
        "page_size": 60,
        "tags": [],
        "type": "community"
    });

    let client = Client::builder().user_agent(USER_AGENT).build()?;

    let resp: Value = client.post(url).json(&payload).send().await?.json().await?;

    let items = &resp["data"]["items"];
    let items = items.clone();

    Ok(items)
}

pub async fn upload_ipfs(url_img: &str, bearer: String) -> Result<String, reqwest::Error> {
    let img_resp = Client::new().get(url_img).send().await?;

    let blob = img_resp.bytes().await.unwrap().to_vec();

    let bearer_str = format!("Bearer {}", bearer);
    let endpoint = format!("{}/upload", STORAGE_URI);

    let client: Value = Client::new()
        .post(endpoint)
        .header("Authorization", bearer_str)
        .header("Content-Type", "image/png")
        .body(blob)
        .send()
        .await?
        .json()
        .await?;

    let cid = &client["value"]["cid"];

    dbg!(&client);

    let cid = cid.clone().to_string();
    Ok(cid)
}

async fn list_cid(bearer: String) -> Result<String, reqwest::Error> {
    let bearer_str = format!("Bearer {}", bearer);

    let client: Value = Client::new()
        .get(STORAGE_URI)
        .header("Authorization", bearer_str)
        .send()
        .await?
        .json()
        .await?;

    let str_pretty = serde_json::to_string_pretty(&client).unwrap();
    Ok(str_pretty)
}

#[tokio::test]
async fn meta_get_1() {
    let get_meta = get_resp_data("8K+gundam+mecha+artstation+unreal+engine")
        .await
        .unwrap();
    dbg!(&get_meta);

    assert_eq!(true, get_meta.is_array())
}

#[tokio::test]
async fn meta_list() {
    use dotenv::dotenv;
    use std::env;

    dotenv().ok();

    let bearer = env::var("IPFS_STORAGE").expect("IPFS not set");

    let val = list_cid(bearer);

    let val = val.await.unwrap();
    dbg!(&val);
    assert_eq!("Test".to_string(), val);
}

#[tokio::test]
async fn meta_ipfs_upload() {
    use dotenv::dotenv;
    use std::env;

    dotenv().ok();

    let bearer = env::var("IPFS_STORAGE").expect("IPFS not set");
    let url_img = "https://cdn1.image.seaart.ai/2023-06-26/38565524222021/0f9a315443af58228e5020fbb7a33a01e8dfdadb.png";

    let cid = upload_ipfs(url_img, bearer).await.unwrap();

    dbg!(&cid);

    assert_eq!("Test".to_string(), cid);
}

// #[test]
// fn replace_test() {
//     let output = "8K gundam mecha artstation unreal engine";
//
//     assert_eq!(
//         output,
//         get_resp_data("8K+gundam+mecha+artstation+unreal+engine")
//     )
// }
