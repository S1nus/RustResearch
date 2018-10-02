extern crate reqwest;
extern crate serde_json;
use serde_json::{Value, Error};
use reqwest::get;
fn main() {
    let body = reqwest::get("http://mysafeinfo.com/api/data?list=englishmonarchs&format=json").unwrap().text().unwrap(); 
    let v : Value = serde_json::from_str(&body).unwrap();
    println!("{}", v[0]["cty"]);
}
