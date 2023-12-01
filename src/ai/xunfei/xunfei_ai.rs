// pub async fn request_api(msg: String) {
//     println!("{}", msg);
// }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct ParameterValue {
    domain: String,
    temperature: f64,
    top_k: i64,
    max_tokens: i64,
    auditing: String,
}

#[derive(Serialize, Deserialize)]
struct Params {
    header: HashMap<String, String>,
    parameter: HashMap<String, ParameterValue>,
    payload: HashMap<String, HashMap<String, Vec<Message>>>,
}

#[allow(dead_code)]
fn gen_params(app_id: &str, question: &str) -> Params {
    let messages = vec![Message {
        role: String::from("user"),
        content: String::from(question),
    }];

    let mut header = HashMap::new();
    header.insert(String::from("app_id"), app_id.to_string());

    let mut parameter = HashMap::new();
    let chat = ParameterValue {
        domain: String::from("general"),
        temperature: 0.8,
        top_k: 6,
        max_tokens: 2048,
        auditing: String::from("default"),
    };
    parameter.insert(String::from("chat"), chat);

    let mut payload = HashMap::new();
    let mut message_map = HashMap::new();
    message_map.insert(String::from("text"), messages);
    payload.insert(String::from("message"), message_map);

    Params {
        header,
        parameter,
        payload,
    }
}

#[allow(dead_code)]
fn main() {
    let app_id = "your_app_id";
    let question = "How are you?";
    let params = gen_params(app_id, question);
    let json_params = serde_json::to_string_pretty(&params).unwrap();
    println!("{}", json_params);
}
