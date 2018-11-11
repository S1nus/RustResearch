extern crate reqwest;
extern crate serde_json;
use serde_json::{Value, Error};
use reqwest::{get, Client};
use std::iter::Map;
use std::thread;

enum OrionStatus {
    Sent,
    Waiting,
    Succeeded,
    Failed,
}

struct ApiRequest {
    status: OrionStatus,
}

struct OrionClient {
    client: Client,
    requests: Vec<ApiRequest>,
}

impl OrionClient {
    fn new() -> OrionClient {
        OrionClient {
            client: Client::builder().build().unwrap(),
            requests: Vec::new()
        }
    }
}

fn main() {
    println!("Blargh");
}
