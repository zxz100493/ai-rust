use crate::ai::dispatch::RequestApi;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::vec;

#[derive(Debug, Serialize, Deserialize)]
struct MsgBody {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Msg<T> {
    pub messages: T,
}

#[async_trait]
impl<T: std::fmt::Debug + ToString + std::marker::Sync> RequestApi for Msg<T> {
    async fn request(&self) {
        println!("baidu impl Ai: {:?}", self.messages);
        // self.request_api();
        let msg = self.messages.to_string();
        request_api(msg).await;
        // self.messages.to_string()
    }
}

pub async fn request_api(msg: String) {
    let token = get_access_token().await;
    let url = format!("https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/completions_pro?access_token={}",token);
    let msg_body = MsgBody {
        role: String::from("user"),
        content: msg,
    };
    let msg_arr = vec![msg_body];

    let msgs = Msg { messages: msg_arr };

    let json_string = serde_json::to_string(&msgs).unwrap();

    println!("您的问题是: {}", json_string);

    let client = get_http_client();
    let body = client
        .post(url)
        .body(json_string)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("Ai: {}", body);
}

#[allow(dead_code)]
fn get_msg() {}

#[allow(unused)]
async fn get_access_token() -> String {
    let url = "https://aip.baidubce.com/oauth/2.0/token";
    let api_key = match std::env::var("BAIDU_CHAT_AI_API_KEY") {
        Ok(val) => val,
        Err(_) => {
            println!("not found api_key");
            return String::from("");
        }
    };

    let api_secret = match std::env::var("BAIDU_CHAT_AI_SECRET_KEY") {
        Ok(val) => val,
        Err(_) => {
            println!("not found api_secret");
            return String::from("");
        }
    };

    let post_data = format!(
        "grant_type=client_credentials&client_id={}&client_secret={}",
        api_key, api_secret
    );

    let client = get_http_client();
    let body = client
        .post(url)
        .body(post_data)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    serde_json::from_str::<serde_json::Value>(&body).unwrap()["access_token"]
        .as_str()
        .unwrap()
        .to_string()
}

fn get_http_client() -> reqwest::Client {
    reqwest::Client::new()
}
