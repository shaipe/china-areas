/**
 * 通过高德地图获取最新的行政区划数据
 */
use serde_derive::Deserialize;
use crate::net::get;

/// 高德地图返回接口数据类型定义
#[derive(Debug, Clone, Deserialize)]
struct Amap{
    status: String,
    info: String,
    infocode: String,
    count: String,
    suggestion: String,
    districts: Vec<District>
}

/// 搜索元素类型定义
#[derive(Debug, Clone, Deserialize)]
struct Suggestion {
    keywords: Vec<String>,
    cities: Vec<String>
}

/// 区域类型定义
#[derive(Debug, Clone, Deserialize)]
struct District {
    citycode: Option<String>,
    adcode: String,
    name: String,
    center: String,
    level: String,
    districts: Vec<District>
}

/// 抓取开始
pub fn start(){
    let provence = fetch_province();
    println!("{}", provence);
}

/// 接口url地址
const URL: &str = "https://restapi.amap.com/v3/config/district?";
/// 高德地图开发者key
const KEY: &str = "a59e1b26770fe9cdda279d8726e97a08";

/// 获取中华人民共和国所有省份
fn fetch_province() -> String {
    // https://restapi.amap.com/v3/config/district?subdistrict=1&key=a59e1b26770fe9cdda279d8726e97a08
    let url = format!("{}subdistrict=1&key={}", URL, KEY);
    let html = get(&url);
    println!("{:?}", html);
    html
}

/// 获取高德地图的数据
fn fetch_amap(keywords: &str, sub: i32) {
    //https://restapi.amap.com/v3/config/district?keywords=武侯区&subdistrict=3&key=a59e1b26770fe9cdda279d8726e97a08
    let url = format!("{}keywords={}&subdistrict={}&key={}", URL, keywords, sub, KEY);
    let html = get(&url);
    println!("{:?}", html);
}