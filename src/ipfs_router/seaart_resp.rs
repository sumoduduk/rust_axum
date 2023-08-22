use reqwest::Client;
use serde_json::{json, Value};

const USER_AGENT :&str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36";

pub fn extract_obj(val: &Value) -> (&str, &str, &str, i32, i32) {
    let image = val["banner"]["url"].as_str().unwrap_or_default();
    let hash_id = val["id"].as_str().unwrap_or_default();
    let prompt = val["prompt"].as_str().unwrap_or_default();
    let width = val["banner"]["width"].as_i64().unwrap_or_default() as i32;
    let height = val["banner"]["height"].as_i64().unwrap_or_default() as i32;

    (image, hash_id, prompt, width, height)
}

pub async fn get_raw_value(
    query_search: String,
    pages: u16,
    tags: Vec<String>,
) -> Result<Value, reqwest::Error> {
    let keyword = query_search.replace('+', " ");
    dbg!(&keyword);
    let url = "https://www.seaart.ai/api/v1/artwork/list";

    let payload = json!({
        "keyword": keyword,
        "order_by": "hot",
        "page": pages,
        "page_size": 60,
        "tags": tags,
        "type": "community"
    });

    let client = Client::builder().user_agent(USER_AGENT).build()?;

    let resp: Value = client.post(url).json(&payload).send().await?.json().await?;
    Ok(resp)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extract_test() {
        let input = dummy_obj();
        let (image, ..) = extract_obj(&input);
        dbg!(image);

        let url =  "https://cdn4.image.seaart.ai/2023-06-21/36919633895493/e99738cbd0c4c84cd6a9f40fb089eba70caea5eb.png";
        assert_eq!(url, image);
    }

    fn dummy_obj() -> Value {
        let obj = json!(
        {
          "id": "ci9i3114msbbe5cs38vg",
          "model_id": "65046a48c1075794ecdb3e8f1ef76f49",
          "model_ver_id": "",
          "prompt": "Positive perspective, Horizontal composition, Verism, reflection light, god rays, blending，Moist high-gloss clothes and body，Gigantic breast，Very thin waist，Photo of Supergirl wearing Supergirl costume，A red cloak is draped over his back，morena，with short ...",
          "local_prompt": "",
          "banner": {
            "width": 1536,
            "height": 2048,
            "url": "https://cdn4.image.seaart.ai/2023-06-21/36919633895493/e99738cbd0c4c84cd6a9f40fb089eba70caea5eb.png",
            "nsfw": 2,
            "is_nsfw_plus": false
          },
          "created_at": "1687363972347",
          "author": {
            "id": "1dad6ec26a4c7291a24f2cc92d21005d",
            "head": "https://cdn5.image.seaart.ai/static/avatar/20230618/fdc7a04c-0e36-4403-a3f4-670251d467f9.jpg",
            "is_follow": false,
            "name": "baiwenyao111",
            "follower_cnt": 68,
            "cc": "CN"
          },
          "parent_art_work_no": "ci9i20h4msbbe5cs224g",
          "meta": null,
          "nsfw": 2,
          "collect": null,
          "liked": false,
          "type": 1,
          "primary": 2,
          "stat": {
            "num_of_like": 1,
            "num_of_collection": 2,
            "num_of_task": 2,
            "num_of_view": 0
          },
          "status": 1,
          "channel": "v5",
          "folder_no": null,
          "green": 2
        }
            );

        obj
    }
}
