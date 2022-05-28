#![allow(dead_code)]

// use http::{Method, Request};
use server::Server;

mod http;
mod server;

fn main() {
    let server = Server::new("127.0.0.1:8000".to_string());
    server.run();
}
