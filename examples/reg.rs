/**
 * 正则表达式测试示例
 */
use regex::Regex;

fn main() {

    let re1 = Regex::new("省|市|自治区").unwrap();
    let res1 = re1.replace("北京市", "");
    let res2 = re1.replace("内蒙古自治区", "");
    let res3 = re1.replace("四川省", "");
    println!("{},{},{}", res1, res2, res3);

    let re = Regex::new("(00)+$").unwrap();
    let res = re.replace("11900000", "");
    println!("{}", res);
}