use axum::{Json, routing::post};
use serde::{Serialize, Deserialize};
use crate::deeplx::request_deepl;


#[derive(Debug, Serialize, Deserialize)]
pub struct RequestParams {
    text: String,
    source_lang: String,
    target_lang: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseParms {
    code: i32,
    msg: String,
    data: String,
}

async fn translate_handler(Json(params): Json<RequestParams>) -> Json<ResponseParms> {
    if let Ok(resp) = request_deepl(params.source_lang, params.target_lang, params.text).await {
        if let Some(data) = resp["result"]["texts"][0]["text"].as_str() {
            Json(ResponseParms {
                code: 200,
                msg: "success".to_owned(),
                data: data.to_owned(),
            })
        } else {
            Json(ResponseParms {
                code: 500,
                msg: resp["error"]["message"].as_str().unwrap().to_owned(),
                data: "".to_string(),
            })
        }
    } else {
        Json(ResponseParms {
            code: 500,
            msg: "internal error".to_string(),
            data: "".to_string(),
        })
    }
}

pub async fn start_server(addr: &str) {
    let app = axum::Router::new()
        .route("/translate", post(translate_handler));
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
