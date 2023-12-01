use ai_rust::ai::baidu::baidu_ai::request_api;
use ai_rust::ai::baidu::baidu_ai::Opt;
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    let Opt { message, .. } = Opt::from_args();
    println!("{:?}", message);
    println!("test sss");

    request_api(message).await;
    // request_api("message").await;
    // let s = get_access_token().await;
    // println!("str is : {}", s);
}
