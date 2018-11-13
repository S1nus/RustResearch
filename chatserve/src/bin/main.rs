extern crate server;

use server::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;
use std::str::from_utf8;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:2003").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let response = format!("blarggg");

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    

}
