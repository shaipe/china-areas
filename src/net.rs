
use reqwest;
use std::io::Read;

pub fn get(url: &str) -> String {
    let mut res = reqwest::get(url).unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    body
}

// pub fn post(url: &str) -> String{
//     let mut res = reqwest::get(url).unwrap();
//     let mut body = String::new();
//     res.read_to_string(&mut body).unwrap();
//     body
// }