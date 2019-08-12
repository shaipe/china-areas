/**
 * 京东的地址数据库
 * https://d.jd.com/area/get?fid=0
 */

use serde::Deserialize;
use lane_net::get_str;
use crate::FileFormat;

const URL: &str = "https://d.jd.com/area/get?fid=";


#[derive(Debug, Clone, Deserialize)]
struct District {
    id: i32,
    name: String,
}


pub fn start(f: FileFormat, sub_level: i32){
    get_province();
    // println!("{}", format!("{}{}", URL, "0"));
    // let url = format!("{}{}", URL, "0");
    // let text = get_str(&url);
    // println!("{:?}", text);
}

fn get_province(){
    let url = format!("{}{}", URL, "0");
    let html = get_str(&url);
    let districts: Vec<District> = serde_json::from_str(&html).unwrap();
    for district in districts {
        if district.id < 100 {
            println!("{:?}", district);
        }
    }
    // println!("{:#?}", uh);
}