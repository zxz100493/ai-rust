use crate::ai::dispatch::RequestApi;
use async_trait::async_trait;
pub struct XunfeiMsg {
    pub msg: String,
}
#[async_trait]
impl RequestApi for XunfeiMsg {
    async fn request(&self) {
        println!("xunfei impl Ai: {}", self.msg);
        // self.msg.clone()
    }
}

pub async fn request_api(msg: String) {
    println!("xunfei Ai: {}", msg);
}
