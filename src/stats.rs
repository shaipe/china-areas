/**
 * 中华人民共和国统计局数据
 * http://www.stats.gov.cn/tjsj/tjbz/tjyqhdmhcxhfdm/
 */
use crate::FileFormat;

pub fn start(f: FileFormat, sub_level: i32) {
    println!("{:?} {}, 正在抓紧开发中...", f, sub_level);
}