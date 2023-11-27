use serde::{Deserialize, Serialize};
use std::vec;

// use std::path::PathBuf;
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    let Opt { message, .. } = Opt::from_args();
    println!("{:?}", message);
    println!("test sss");
    request_api(message).await;
    // let s = get_access_token().await;
    // println!("str is : {}", s);
}

#[derive(Debug, StructOpt)]
#[allow(dead_code)]
#[structopt(name = "ai talk by rust", about = "talk with ai by rust api")]
struct Opt {
    /// 你的问题
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MsgBody {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Msg<T> {
    messages: T,
}

async fn request_api(msg: String) {
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
