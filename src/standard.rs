/**
 * 将根据多种数据源对区域数据进行合并
 */

/// 标准行政区结构
struct District {
    // 行政区编码(国标)
    code: String,
    // 行政区名称
    name: String,
    // 父级行政区编码
    parent_code: String,
    // 行政区级次
    level: i32,
    // 顺序
    reorder: i32,
    // 经度
    longitude: u64,
    // 纬度
    latitude: u64,
    // 下级行政区
    districts: Vec<District>
}

impl District {

    /// 创建一个新空的行政区对象
    fn new() -> Self {
        District {
            code: String::new(),
            name: String::new(),
            parent_code: String::new(),
            level: 1,
            reorder: 1,
            longitude: 0.0,
            latitude: 0.0,
            districts: vec![]
        }
    }
}


