/**
 * 中华人民共和国民政部数据
 * http://www.mca.gov.cn/article/sj/xzqh/2019
 * https://www.mca.gov.cn/article/sj/xzqh/1980/202203/20220300040708.shtml
 */
use crate::{FileFormat, get_str};
use scraper::{Html, Selector};

pub fn start(f: FileFormat, sub_level: i32) {
    println!("{:?} {}, 正在抓紧开发中...", f, sub_level);

    parse_html("http://www.mca.gov.cn/article/sj/xzqh/2019/201908/201908271607.html");
}

/// 获取和解析html
pub fn parse_html(url: &str) {

    let html = get_str(url);
    let doc = Html::parse_document(&html);
    let selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let mut res: Vec<Vec<String>> = vec![];
    for tr in doc.select(&selector) {
        // println!("{:?}", tr.text());
        // let x: Vec<String> = tr.text().map(|text| text.to_string()).collect();
        // println!("{:?}", x);
        let mut x = 0;
        let mut xv: Vec<String> = vec![];
        for td in tr.select(&td_selector) {
            if x == 1 || x == 2 {
                // xv.push(td.text().);
            }
            x += 1;
        }
        println!("{:?}", xv);
        res.push(xv);
    }

    println!("{:?}", html);
}