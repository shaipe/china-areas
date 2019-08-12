/**
 * 通过高德地图获取最新的行政区划数据
 * http://lbs.amap.com/api/webservice/guide/api/district
 */
use serde_derive::{Deserialize, Serialize};
use lane_net::get_str;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

/// 高德地图返回接口数据类型定义
#[derive(Debug, Clone, Deserialize, Serialize)]
struct Amap{
    status: String,
    info: String,
    infocode: String,
    count: String,
    suggestion: Suggestion,
    districts: Vec<District>
}

/// 搜索元素类型定义
#[derive(Debug, Clone, Deserialize, Serialize)]
struct Suggestion {
    keywords: Vec<String>,
    cities: Vec<String>
}

/// 区域类型定义
#[derive(Debug, Clone, Deserialize, Serialize)]
struct District {
    // citycode: Option<Vec<String>>,
    adcode: String,
    name: String,
    center: String,
    level: String,
    districts: Vec<District>
}

/// 高德地图数据结构扩展
impl Amap {

    /// 获取一个默认数据
    fn new() -> Self{
        Amap{
            status: "".to_owned(),
            info: "ok".to_owned(),
            infocode: "".to_owned(),
            count: "0".to_owned(),
            suggestion: Suggestion {
                keywords: vec![],
                cities:vec![]
            },
            districts: vec![]
        }
    }
}

use crate::FileFormat;

/// 抓取开始
pub fn start(f: FileFormat, sub_level: i32){
    println!("正在通过高德接口获取数据...");
    // 获取所有的数据
    let province = fetch_province(sub_level);
    println!("获取数据完成,正在对数据进行分析处理...");
    // println!("{:#?}", province);
    let a: Amap = match serde_json::from_str(&province){
        Ok(z) => z,
        Err(e) => {
            println!("{:?}", e);
            Amap::new()
        }
    };
    let ext = match f {
        FileFormat::Sql => "sql",
        FileFormat::Csv => "csv",
        _ => "json"
    };

    let file_name = format!("./data/areas.{}", ext);
    // println!("{}", file_name);
    // return;
    let mut res = analyze_districts(a.districts, "-1", &f.clone());

    // 给定表结构
    res.insert(0, "replace into cor_Region (CodeId, ParentId, Name) VALUES ".to_owned());
    
    let mut file = match File::create(file_name.clone()) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };

    let res_str = res.join("\n");
    match file.write_all(res_str.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to : {}", why)
        },
        Ok(_) => {println!("successfully wrote to {}", file_name)},
    };
    
    println!("高德地图行政区划接口分析结束");
}

/// 分析区域组合
fn analyze_districts(districts: Vec<District>, parent_code: &str, fmat: &FileFormat) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    for district in districts {
        let dis = district.clone();
        let code = district.adcode;
        let dis_res = analyze_district(dis, fmat.clone(), parent_code);
        res.push(dis_res);
        if district.districts.len() > 0 {
            let mut diss_res = analyze_districts(district.districts, &code, fmat);
            res.append(&mut diss_res);
        }
    }
    res
}

/// 单个区域数据分析
fn analyze_district(district: District, format: FileFormat, parent_code: &str) -> String {
    let re_zero = Regex::new("(0+)$").unwrap();
    let p = re_zero.replace(parent_code, "");
    let code = re_zero.replace(&district.adcode, "");
    match format {
        FileFormat::Sql => format!("({},{},{}),", code, p, district.name),
        FileFormat::Csv => format!("{},{},{},", code, p, district.name),
        FileFormat::Json => serde_json::to_string_pretty(&district).unwrap(),
        _ => String::new()
    }
}


/// 接口url地址
const URL: &str = "https://restapi.amap.com/v3/config/district?";
/// 高德地图开发者key
const KEY: &str = "a59e1b26770fe9cdda279d8726e97a08";

/// 获取中华人民共和国所有省份
fn fetch_province(sub_level: i32) -> String {
    // https://restapi.amap.com/v3/config/district?subdistrict=1&key=a59e1b26770fe9cdda279d8726e97a08
    // 获取出中国所有的省市区县
    let url = format!("{}subdistrict={}&key={}", URL, sub_level, KEY);
    let html = get_str(&url);
    // println!("{:?}", html);
    html
}

/// 获取高德地图的数据
fn fetch_amap(keywords: &str, sub: i32) {
    //https://restapi.amap.com/v3/config/district?keywords=武侯区&subdistrict=3&key=a59e1b26770fe9cdda279d8726e97a08
    let url = format!("{}keywords={}&subdistrict={}&key={}", URL, keywords, sub, KEY);
    let html = get_str(&url);
    println!("{:?}", html);
}