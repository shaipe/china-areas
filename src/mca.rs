/**
 * 中华人民共和国民政部数据
 * http://www.mca.gov.cn/article/sj/xzqh/2019
 */
use crate::FileFormat;

pub fn start(f: FileFormat, sub_level: i32) {
    println!("{:?} {}, 正在抓紧开发中...", f, sub_level);
}