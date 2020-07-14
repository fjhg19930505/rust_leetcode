extern crate rand;

use crypto::digest::Digest;
use crypto::md5::Md5;
use std::time::SystemTime;

mod leetcode;
mod normal_test;
fn main() {
    /*let start = SystemTime::now();
    let mut result = String::from("文和");
    for i in 0 .. 100000000 {
        result = md5(result);
    }

    let end = SystemTime::now();
    let time = end.duration_since(start).expect("error!");
    println!("result = {}, cal_time = {}", result, time.as_millis());*/

    let s = String::from("LEETCODEISHIRING");
    let numRows = 4;

    let result = leetcode::tencent::z_transfer::convert(s, numRows);
    println!("{}", result);
}

pub fn md5<S:Into<String>>(input: S) -> String {
    let mut md5 = Md5::new();
    md5.input_str(&input.into());
    md5.result_str()
}



