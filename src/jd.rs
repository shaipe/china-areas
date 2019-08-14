/**
 * 京东的地址数据库
 * https://d.jd.com/area/get?fid=0
 */

use serde_derive::{Deserialize, Serialize};
use lane_net::get_str;
use crate::FileFormat;
use crate::util::write_file;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::convert::TryFrom;

/// 京东地址数据接口地址
const URL: &str = "https://d.jd.com/area/get?fid=";

/// 地区结构定义
#[derive(Debug, Clone, Deserialize, Serialize)]
struct District {
    id: i32,
    name: String,
    parent_id: Option<i32>,
    level: Option<i32>,
    reorder: Option<i32>,
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

        let level = match dist.level {
            Some(l) => l,
            None => 1
        };

        let reorder = match dist.reorder {
            Some(l) => l,
            None => 0
        };

        match fmat {
            FileFormat::Sql => format!("({},'{}',{},{},{})", dist.id, dist.name, parent_id, level, reorder),
            FileFormat::Csv => format!("{},{},{},{},{}", dist.id, dist.name, parent_id, level, reorder),
            FileFormat::Json => serde_json::to_string_pretty(&dist).unwrap()
        }
    }
}

// static mut distrcts_str: Option<Vec<String>> = None;
/// 京东数据获取入口
pub fn start(f: FileFormat, sub_level: i32){
    println!("{:?} {}, 开始京东数据分析...", f, sub_level);
    
    // 根据格式进行处理
    let res = match f {
        FileFormat::Sql | FileFormat::Csv => get_districts_no_json(f.clone(), sub_level),
        FileFormat::Json => get_districts_json(sub_level)
    };
    
    // 把结果写入文件
    write_file("jd", res, sub_level, f);

    println!("京东行政区划接口分析结束");
}

fn get_districts_no_json (f: FileFormat, sub_level: i32) -> Vec<String> {

    let mut res: Vec<String> = vec![];

    let provinces = get_province("china");
    let mut x = get_districts_str(0, provinces.clone(), 1, &f);
    res.append(&mut x);

    // 给定表结构
    match f {
        FileFormat::Sql => res.insert(0, "replace into cor_Region (CodeId, ParentId, Name) VALUES ".to_owned()),
        _ => {}
    };

    if sub_level > 1 {
        
        // let m = MultiProgress::new();
        let sty = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-");
        // 获取数组的长度转换为u64
        let b: u64 = u64::try_from(provinces.len()).unwrap();
        let pb = ProgressBar::new(b);
        pb.set_style(sty.clone());
        // let mut px = 0;

        // 获取二级城市
        for prov in provinces {
            pb.inc(1);
            // println!("{}", prov.id);
            let cities = get_districts(prov.id);
            let mut cities_str = get_districts_str(prov.id, cities.clone(), 2, &f);
            res.append(&mut cities_str);

            // 获取区限级的数据
            if sub_level > 2 {
                for city in cities {
                    let ars = get_districts(city.id);
                    let mut ars_str = get_districts_str(city.id, ars.clone(), 3, &f);
                    res.append(&mut ars_str);

                    // 获取镇级数据
                    if sub_level > 3 {
                        
                        for a in ars {
                            let towers = get_districts(a.id);
                            let mut towers_str = get_districts_str(a.id, towers.clone(), 4, &f);
                            res.append(&mut towers_str);
                        }
                    }
                }
            }
        }
        // m.join_and_clear().unwrap();
        pb.finish_with_message("done");
    }

    res
}

fn get_districts_json(sub_level: i32) -> Vec<String> {
    let provinces = get_province("china");
    let mut res: Vec<String> = vec![];


    let sty = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .progress_chars("##-");
    // 获取数组的长度转换为u64
    let b: u64 = u64::try_from(provinces.len()).unwrap();
    let pb = ProgressBar::new(b);
    pb.set_style(sty.clone());

    let mut p_order = 1;
    // 开始组装
    for mut p in provinces {
        pb.inc(1);
        if sub_level > 1 {
            let cs = get_districts(p.id);
            let mut c_order = 1;
            // 此处用来记录已修改的子级对象
            let mut mcs:Vec<District> = vec![];
            for mut c in cs {
                // 获取区县
                if sub_level > 2 {
                    let a_s = get_districts(c.id);
                    let mut mas: Vec<District> = vec![];
                    let mut a_order = 1;    
                    for mut a in a_s.clone() {
                        // 获取乡镇
                        if sub_level > 3 {
                            let ts = get_districts(a.id);
                            let mut mts: Vec<District> = vec![];
                            let mut t_order = 1;
                            for mut t in ts.clone() {
                                t.level = Some(4);
                                t.reorder = Some(t_order);
                                t.parent_id = Some(a.id);
                                mts.push(t);
                                t_order += 1;
                            }
                            a.districts = Some(ts);
                        }
                        //
                        a.level = Some(3);
                        a.reorder = Some(a_order);
                        a.parent_id = Some(c.id);
                        mas.push(a);
                        a_order += 1;
                    }

                    c.districts = Some(a_s);
                }
                c.level = Some(2);
                c.reorder = Some(c_order);
                c.parent_id = Some(p.id);
                mcs.push(c);
                c_order += 1;
            }
            // println!("{:?}", cs.clone());
            
            p.districts = Some(mcs);
        }

        p.level = Some(1);
        p.reorder = Some(p_order);
        p_order += 1;

        let json_res = serde_json::to_string_pretty(&p);
        let json_str = match json_res {
            Ok(s) => s,
            Err(e) => String::from("xx")
        };
        res.push(json_str);
    }
    res
}



/// 获取区域数据为对象和
fn get_districts_str(id: i32, districts: Vec<District>, level: i32, fmat: &FileFormat) -> Vec<String> { 
    let mut dists_str: Vec<String> = vec![];
    let mut reorder: i32 = 1;
    for mut dist in districts.clone() {
        dist.parent_id = Some(id);
        dist.reorder = Some(reorder);
        dist.level = Some(level);
        dists_str.push(dist.to_str(fmat));
        reorder += 1;
    }
    dists_str
}

fn get_districts(id: i32) -> Vec<District>{
    let url = format!("{}{}", URL, id);
    let html = get_str(&url);
    // if html == "" {
    //     vec![]
    // }
    // else{
        let districts: Vec<District> = match serde_json::from_str(&html) {
            Ok(j) => j,
            Err(_) => vec![]
        };
        districts
    // }
    
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