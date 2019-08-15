/**
 * 将根据多种数据源对区域数据进行合并
 */

use crate::FileFormat;
use crate::amap::HashAmap;
use serde_derive::{Deserialize, Serialize};

/// 标准行政区结构
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StdDistrict {
    // 行政区编码(国标)
    pub code: String,
    // 行政区名称
    pub name: String,
    // 父级行政区编码
    pub parent_code: String,
    // 行政区级次
    pub level: i32,
    // 顺序
    pub reorder: i32,
    // 经度
    pub longitude: Option<f32>,
    // 纬度
    pub latitude: Option<f32>,
    // 下级行政区
    pub districts: Option<Vec<StdDistrict>>
}

impl StdDistrict {

    /// 创建一个新空的行政区对象
    pub fn new() -> Self {
        StdDistrict {
            code: String::new(),
            name: String::new(),
            parent_code: String::new(),
            level: 1,
            reorder: 1,
            longitude: Some(0.0),
            latitude: Some(0.0),
            districts: Some(vec![])
        }
    }

    /// 将区域数据转换为字符串
    pub fn to_str(&self, fmat: &FileFormat) -> String {
        let dist = self;
        
        match fmat {
            FileFormat::Sql => format!("({},'{}',{},{},{})", dist.code, dist.name, dist.parent_code, dist.level, dist.reorder),
            FileFormat::Csv => format!("{},{},{},{},{}", dist.code, dist.name, dist.parent_code, dist.level, dist.reorder),
            FileFormat::Json => serde_json::to_string_pretty(&dist).unwrap()
        }
    }
}


pub fn start(f: FileFormat, sub_level: i32) {
    println!("{:?} {}, 开始京东数据分析...", f, sub_level);
    
    // 从高德中获取名称和行政编码
    let mut ap = HashAmap::new();
    let maps = ap.get_maps(sub_level);

    println!("{:?}", maps);
    // 根据格式进行处理
    // let res = match f {
    //     FileFormat::Sql | FileFormat::Csv => get_districts_no_json(f.clone(), sub_level),
    //     FileFormat::Json => get_districts_json(sub_level)
    // };
    
    // 把结果写入文件
    // write_file("jd", res, sub_level, f);

    println!("京东行政区划接口分析结束");
}

