/**
 * 京东的地址数据库
 * https://d.jd.com/area/get?fid=0
 */

use serde_derive::{Deserialize, Serialize};
use lane_net::get_str;
use crate::FileFormat;
use crate::util::write_file;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

/// 京东地址数据接口地址
const URL: &str = "https://d.jd.com/area/get?fid=";

/// 地区结构定义
#[derive(Debug, Clone, Deserialize, Serialize)]
struct District {
    id: i32,
    name: String,
    parent_id: Option<i32>,
    districts: Option<Vec<District>>
}

/// 地区信息结构扩展
impl District {
    
    /// 将区域数据转换为字符串
    fn to_str(&self, fmat: &FileFormat) -> String {
        let dist = self;
        
        let parent_id = match dist.parent_id {
            Some(id) => id,
            None => 1
        };

        match fmat {
            FileFormat::Sql => format!("({},'{}',{}),", dist.id, dist.name, parent_id),
            FileFormat::Csv => format!("{},{},{}", dist.id, dist.name, parent_id),
            FileFormat::Json => serde_json::to_string_pretty(&dist).unwrap()
        }
    }
}

// static mut distrcts_str: Option<Vec<String>> = None;
/// 京东数据获取入口
pub fn start(f: FileFormat, sub_level: i32){
    println!("{:?} {}, 开始京东数据分析...", f, sub_level);
    
    // 根据格式进行处理
    // match f {
    //     FileFormat::Sql | FileFormat::Csv => {
    //         unsafe{
    //             distrcts_str = Some(vec![]);
    //         }
    //     }
    //     FileFormat::Json => {}
    // }
    
    let mut res: Vec<String> = vec![];

    let provinces = get_province("china");
    let mut x = get_districts_str(0, provinces.clone(), &f);
    res.append(&mut x);

    // 给定表结构
    match f {
        FileFormat::Sql => res.insert(0, "replace into cor_Region (CodeId, ParentId, Name) VALUES ".to_owned()),
        _ => {}
    };

    if sub_level > 1 {
        
        let m = MultiProgress::new();
        let sty = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-");

        let pb = m.add(ProgressBar::new(provinces.len()));
        pb.set_style(sty.clone());
        let px = 1;

        // 获取二级城市
        for prov in provinces {
            let cities = get_districts(prov.id);
            let mut cities_str = get_districts_str(prov.id, cities.clone(), &f);
            res.append(&mut cities_str);

            // 获取区限级的数据
            if sub_level > 2 {
                for city in cities {
                    let ars = get_districts(city.id);
                    let mut ars_str = get_districts_str(city.id, ars.clone(), &f);
                    res.append(&mut ars_str);

                    // 获取镇级数据
                    if sub_level > 3 {
                        for a in ars {
                            let towers = get_districts(a.id);
                            let mut towers_str = get_districts_str(a.id, towers.clone(), &f);
                            res.append(&mut towers_str);
                        }
                        
                    }
                }
            }

            pb.set_message(&format!("item #{}", px + 1));
            pb.inc(1);
        }
    }

    // 把结果写入文件
    write_file("jd", res, f);

    println!("京东行政区划接口分析结束");
}




/// 获取区域数据为对象和
fn get_districts_str(id: i32, districts: Vec<District>, fmat: &FileFormat) -> Vec<String> { 
    let mut dists_str: Vec<String> = vec![];
    for mut dist in districts.clone() {
        dist.parent_id = Some(id);
        dists_str.push(dist.to_str(fmat));
    }
    dists_str
}

fn get_districts(id: i32) -> Vec<District>{
    let url = format!("{}{}", URL, id);
    let html = get_str(&url);
    let districts: Vec<District> = serde_json::from_str(&html).unwrap();
    districts
}

/// 获取中华人民共和国省份
fn get_province(scope: &str) -> Vec<District>{
    let url = format!("{}{}", URL, "0");
    let html = get_str(&url);
    let districts: Vec<District> = serde_json::from_str(&html).unwrap();
    if scope == "all" {
        districts
    }
    else{
        let mut dists: Vec<District> = vec![];
        for mut district in districts {
            if district.id < 100 {
                district.parent_id = Some(1);
                dists.push(district)
            }
        }
        dists
    }
}