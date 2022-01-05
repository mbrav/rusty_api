// Example request
// GET / HTTP/1.1
// Host: developer.mozilla.org
// Accept-Language: fr

use http::Method;
use http::Request;
use server::Server;

mod http;
mod server;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
