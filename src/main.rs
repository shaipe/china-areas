
extern crate reqwest;
extern crate lane_net;

mod amap;
mod jd;
use std::env;

/// 支持的格式
#[derive(Clone)]
pub enum FileFormat {
    Json,
    Csv,
    Sql
}

fn main() {
    // jd::start();
    // 获取输入的参,参数为待处理的路径
    let args: Vec<String>= env::args().collect();

    let fmat_str = if args.len() > 1 {
        &args[1]
    }
    else{
        "sql"
    };
    
    let fmat = match fmat_str {
        "csv" => FileFormat::Csv,
        "json" => FileFormat::Json,
        _ => FileFormat::Sql,
    };

    let level = if args.len() > 2 {
        let x = &args[2];
        x.parse::<i32>().unwrap()
    }
    else{
        3
    };

    amap::start(fmat, level);
    println!("程序执行结束!");
}

