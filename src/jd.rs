/**
 * 京东的地址数据库
 * https://d.jd.com/area/get?fid=0
 */

use crate::net::get;

const URL: &str = "https://d.jd.com/area/get?fid=";


#[derive(Debug, Clone)]
struct District {
    id: i32,
    name: String,
}

pub fn start(){
    println!("{}", format!("{}{}", URL, "0"));
    let url = format!("{}{}", URL, "0");
    let text = get(&url);
    println!("{:?}", text);
}