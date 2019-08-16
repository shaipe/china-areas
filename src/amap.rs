/**
 * 通过高德地图获取最新的行政区划数据
 * http://lbs.amap.com/api/webservice/guide/api/district
 */
use serde_derive::{Deserialize, Serialize};
use lane_net::get_str;
use regex::Regex;
use std::collections::HashMap;
use crate::util::write_file;

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
    parent_code: Option<String>,
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
    
    let mut res = match f {
        FileFormat::Json => {
            to_json(a.districts)
        }
        _ => Analyze::new().analyze_districts(a.districts, "-1", 0, &f.clone())
    };

    // 给定表结构
    match f {
        FileFormat::Sql => res.insert(0, "replace into cor_Region (CodeId, ParentId, Name, Layer, Reorder) VALUES ".to_owned()),
        _ => {}
    };

    // 把结果写入文件
    write_file("amap", res, sub_level, f);
    
    println!("高德地图行政区划接口分析结束");
}

fn to_json(districts: Vec<District>) -> Vec<String> {
    let mut rs: Vec<String> = vec![];
    for d in districts {
        rs.push(serde_json::to_string_pretty(&d).unwrap());
    }
    rs
}

struct Analyze {
    district_count: i32
}

impl Analyze {

    fn new() -> Self {
        Analyze{
            district_count: 0
        }
    }

    /// 分析区域组合
    fn analyze_districts(&mut self, districts: Vec<District>, parent_code: &str, level: i32, fmat: &FileFormat) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        let mut reorder: i32 = 1;
        for district in districts {
            let dis = district.clone();
            let code = district.adcode;
            let dis_res = self.analyze_district(dis, fmat.clone(), parent_code, level, reorder);
            res.push(dis_res);
            if district.districts.len() > 0 {
                self.district_count = 0;
                let mut diss_res = self.analyze_districts(district.districts, &code, level+1, fmat);
                res.append(&mut diss_res);
            }
            reorder += 1;
        }
        res
    }

    /// 单个区域数据分析
    fn analyze_district(&mut self, district: District, format: FileFormat, parent_code: &str, level: i32, reorder: i32) -> String {
        let re_zero = Regex::new("(00)+$").unwrap();
        let p = re_zero.replace(parent_code, "");
        let code = re_zero.replace(&district.adcode, "");
        let mut codex = format!("{}", code);
        if code == p {
            self.district_count +=1;
            // 不足两位用0在前面补齐
            let c = if self.district_count < 10 {
                format!("0{}", self.district_count)
            }
            else{
                self.district_count.to_string()
            };

            codex = format!("{}{}", code, c);
        }

        let re_province = Regex::new("省|市|维吾尔自治区|回族自治区|自治区|壮族自治区").unwrap();
        let re_city = Regex::new("市").unwrap();
        let name =  match level {
            1 => format!("{}", re_province.replace(&district.clone().name, "")),
            2 => format!("{}", re_city.replace(&district.clone().name, "")),
            _ => district.clone().name
        };


        match format {
            FileFormat::Sql => format!("({},{},'{}',{},{})", codex, p, name, level, reorder),
            FileFormat::Csv => format!("{},{},{},{},{}", codex, p, name, level, reorder),
            FileFormat::Json => serde_json::to_string_pretty(&district).unwrap()
        }
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

// /// 获取高德地图的数据
// fn fetch_amap(keywords: &str, sub: i32) {
//     //https://restapi.amap.com/v3/config/district?keywords=武侯区&subdistrict=3&key=a59e1b26770fe9cdda279d8726e97a08
//     let url = format!("{}keywords={}&subdistrict={}&key={}", URL, keywords, sub, KEY);
//     let html = get_str(&url);
//     println!("{:?}", html);
// }

/// 高德地图的行政区划字典对象
pub struct HashAmap{
    maps: HashMap<String, String>
}



impl HashAmap {

    pub fn new() -> Self {
        HashAmap {
            maps: HashMap::new()
        }
    }

    /// 获取对应级次的所有行政区名称和编码
    pub fn get_maps(&mut self, level: i32) -> HashMap<String, String> {
        // let mut codes: HashMap<String, String> = HashMap::new();
        use crate::util::read_content;

        println!("正在通过高德接口获取数据...");
        let content = read_content("amap", level);
        let provinces: Vec<District> = serde_json::from_str(&content).unwrap();
        
        // // 获取所有的数据
        // let province = fetch_province(level);
        // println!("获取数据完成,正在对数据进行分析处理...");
        // // println!("{:#?}", province);
        // let a: Amap = match serde_json::from_str(&province){
        //     Ok(z) => z,
        //     Err(e) => {
        //         println!("{:?}", e);
        //         Amap::new()
        //     }
        // };

        // 采用递归的方式处理子级
        self.get_districts_map(provinces, 0, "".to_owned());
        // println!("{:?}", self.maps);
        self.maps.clone()
    }

    /// 递归的形式处理获取字典
    fn get_districts_map(&mut self, dists: Vec<District>, level: i32, p_code: String) {
    
        for dist in dists {
            let re_zero = Regex::new("(00)+$").unwrap();
            let re_province = Regex::new("省|市|维吾尔自治区|回族自治区|自治区|壮族自治区").unwrap();
            // let re_city = Regex::new("市|自治州").unwrap();
            let name =  match level {
                1 => format!("{}", re_province.replace(&dist.name, "")),
                // 2 => format!("{}", re_city.replace(&dist.name, "")),
                _ => dist.name
            };
            let code = re_zero.replace(&dist.adcode, "");
            // let pcode = match dist.parent_code {
            //     Some(c) => c,
            //     None => "".to_owned()
            // };
            let parent_code = re_zero.replace(&p_code, "");
            // println!("{},{}", name, format!("{},{}", code.clone(), parent_code));
            self.maps.insert(name, format!("{},{}", code.clone(), parent_code));
            // 递归进行子级处理
            if dist.districts.len() > 0 {
                // println!("{}", code);
                self.get_districts_map(dist.districts, level+1, format!("{}", code.clone()));
            }
        }
    }
}
