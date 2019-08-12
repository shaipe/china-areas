
use regex::Regex;

fn main() {
    let re = Regex::new("(0+)$").unwrap();
    let res = re.replace("11000", "");
    println!("{}", res);
}