use rand::prelude::*;
use chrono::prelude::*;

pub async fn request_deepl(source_lang: String, target_lang: String, translate_text: String) -> Result<serde_json::Value, reqwest::Error> {
    let post_json = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "LMT_handle_texts",
        "id": get_random_number(),
        "params": {
            "splitting": "newlines",
            "timestamp": get_timestamp(get_icount(&translate_text)),
            "lang": {
                "source_lang_user_selected": &source_lang,
                "target_lang": &target_lang,
            },
            "texts": [{
                "text": &translate_text,
                "requestAlternatives": 3,
            }]
        },
    });
    let mut post_str = post_json.to_string();
    if (post_json["id"].as_i64().unwrap() + 5) % 29 == 0 || (post_json["id"].as_i64().unwrap() + 3) % 13 == 0 {
        post_str = post_str.replacen("\"method\":\"", "\"method\" : \"", 1);
    }
    else {
        post_str = post_str.replacen("\"method\":\"", "\"method\": \"", 1);
    }
                
    let deepl_resp: serde_json::Value = reqwest::Client::new()
        .post("https://www2.deepl.com/jsonrpc")
        // .json(&post_json)
        .body(post_str)
        .header("Content-Type", "application/json")
        .header("Accept", "*/*")
        .header("x-app-os-name", "iOS")
        .header("x-app-os-version", "16.3.0")
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("Accept-Encoding", "gzip, deflate, br")
        .header("x-app-device", "iPhone13,2")
        .header("User-Agent", "DeepL-iOS/2.6.0 iOS 16.3.0 (iPhone13,2)")
        .header("x-app-build", "353933")
        .header("x-app-version", "2.6")
        .header("Connection", "keep-alive")
        .send()
        .await?
        .json()
        .await?;
    Ok(deepl_resp)
}


fn get_random_number() -> i64 {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(8300000..=9199999);
    num * 1000
}

fn get_icount(translate_text: &str) -> i64 {
    translate_text.matches("i").count() as i64
}

fn get_timestamp(mut i_count: i64) -> i64 {
    let ts = Local::now().timestamp_millis();
    if i_count != 0 {
        i_count += 1;
        ts - ts % i_count + i_count
    } else {
        ts
    }
}

