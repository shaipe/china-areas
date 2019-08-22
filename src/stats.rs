/**
 * 中华人民共和国统计局数据
 * http://www.stats.gov.cn/tjsj/tjbz/tjyqhdmhcxhfdm/
 */
use crate::FileFormat;
use lane_net::get_str;

pub fn start(f: FileFormat, sub_level: i32) {
    println!("{:?} {}, 正在抓紧开发中...", f, sub_level);
    get_province();
}

pub fn get_province() {
    println!("{:?}", "sdad");
    let html = get_str("http://www.stats.gov.cn/tjsj/tjbz/tjyqhdmhcxhfdm/2018/index.html");

    println!("{:?}", html);
}