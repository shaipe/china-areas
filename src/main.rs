/**
 * 中华人民共和国行政区划数据
 * 可支持json, csv, sql三种格式输出
 * authors:// shaipe
 */

extern crate reqwest;
extern crate lane_net;

/// 项目内部模块引入
mod amap;
mod jd;
mod mca;
mod stats;

// 
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

    // 获取数据来源参数
    let source = if args.len() > 1 {
        &args[1]
    }
    else{
        "amap"
    };

    // 获取输出的数据格多参数
    let fmat_str = if args.len() > 2 {
        &args[2]
    }
    else{
        "sql"
    };
    
    let fmat = match fmat_str {
        "csv" => FileFormat::Csv,
        "json" => FileFormat::Json,
        _ => FileFormat::Sql,
    };

    // 获取需要获取到第几级的参数
    let level = if args.len() > 3 {
        let x = &args[3];
        x.parse::<i32>().unwrap()
    }
    else{
        3
    };

    match source {
        "jd" => jd::start(fmat, level),
        "mca" => mca::start(fmat, level),
        "stats" => stats::start(fmat, level),
        _ => amap::start(fmat, level)
    };
    
    println!("程序执行结束!");
}

