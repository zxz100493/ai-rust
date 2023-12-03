use super::baidu;
use crate::ai::xunfei;
use async_trait::async_trait;

// enum AiMode {
//     BaiduMode,
//     XunfeiMode,
// }

// impl AiMode {
//     fn as_str(&self) -> &'static str {
//         match *self {
//             // AiMode::BaiduMode => crate::ai::const_num::BAIDU_MODE_NAME,
//             AiMode::BaiduMode => super::const_num::BAIDU_MODE_NAME,
//             AiMode::XunfeiMode => super::const_num::XUNFEI_NAME,
//         }
//     }
// }

pub async fn request_api_by_mode(mode_name: &str, m: &str) {
    println!("mode name is : {}", mode_name);

    match mode_name {
        super::const_num::BAIDU_MODE_NAME => {
            println!("baidu mode");
            let msg = baidu::baidu_ai::Msg {
                messages: m.to_string(),
            };
            let _ = msg.request();
            // baidu::baidu_ai::request_api(message.to_string()).await;
        }
        super::const_num::XUNFEI_NAME => {
            println!("xunfei mode");
            let msg = xunfei::xunfei_ai::XunfeiMsg { msg: m.to_string() };
            let _ = msg.request();
            // xunfei::xunfei_ai::request_api(m.to_string()).await;
        }
        _ => {
            println!("default baidu mode");
            baidu::baidu_ai::request_api(m.to_string()).await;
        }
    };
}
#[async_trait]
pub trait RequestApi {
    async fn request(&self);
}
