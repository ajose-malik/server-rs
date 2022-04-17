mod http;
mod server;
use http::Request;
use http::Method;
use server::Server;

fn main() {
    // let get = Method::GET;
    // let delete = Method::DELETE;
    // let post = Method::POST;
    // let put = Method::PUT;

    let server = Server::new("127.0.0.1:8000".to_string());
    server.run();
}
