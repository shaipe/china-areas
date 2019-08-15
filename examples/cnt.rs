
use std::fs::read_to_string;

fn main() {
    println!("{:?}", read_content("jd", 2));
}

/// 读取文本文件中的内容
fn read_content(source: &str, level: i32) -> String {
    let file_name = format!("./data/{}/areas-level{}.json", source, level);
  
    let content = match read_to_string(file_name) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(cnt) => cnt,
    };

    content
}