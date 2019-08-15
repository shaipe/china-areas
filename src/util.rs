
use crate::FileFormat;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::create_dir_all;

/// 把数据写入到文件中
pub fn write_file(source: &str, res: Vec<String>, level: i32, f: FileFormat) {
    
    // 判断目录是否存在,不存在即创建目录
    let file_dir = format!("./data/{}", source);
    let p = Path::new(&file_dir);
    if !p.exists(){
        let _ = create_dir_all(p);
    }

    // 创建文件写入对象
    let file_name = format!("{}/areas-level{}.{}", file_dir, level, f.as_str());
    let mut file = match File::create(file_name.clone()) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };

    // 对格式进行判断
    let res_str = match f {
        FileFormat::Sql=> res.join(",\n"),
        FileFormat::Json => {
            let mut x: String = String::from("[");
            x.push_str(&res.join(",\n"));
            x.push_str("]");
            x
        },
        FileFormat::Csv => res.join("\n")
    };

    // 将数据流写入文件
    match file.write_all(res_str.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to : {}", why)
        },
        Ok(_) => {println!("successfully wrote to {}", file_name)},
    };
}