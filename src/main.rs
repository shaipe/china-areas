
extern crate reqwest;
mod amap;
mod jd;
mod net;


fn main() {
    jd::start();
    amap::start();
    println!("Hello, world!");
}

