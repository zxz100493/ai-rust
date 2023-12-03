use ai_rust::ai::command::Opt;
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    let Opt { message, name, .. } = Opt::from_args();
    println!("{:?}", message);
    println!("test sss");

    ai_rust::ai::dispatch::request_api_by_mode(name.as_str(), message.as_str()).await;
    // request_api("message").await;
    // let s = get_access_token().await;
    // println!("str is : {}", s);
}
