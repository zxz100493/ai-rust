// use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[allow(dead_code)]
#[structopt(name = "ai talk by rust", about = "talk with ai by rust api")]
pub struct Opt {
    /// 你的问题
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    pub message: String,

    /// 模型
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    pub name: String,
}
