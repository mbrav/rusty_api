// Example request
// GET / HTTP/1.1
// Host: developer.mozilla.org
// Accept-Language: fr

use http::request::Request;
use server::Server;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

mod server {
    pub struct Server {
        addr: String,
    }

    impl Server {
        pub fn new(addr: String) -> Self {
            Self { addr: addr }
        }

        pub fn run(self) {
            println!("Server running on {}", self.addr)
        }
    }
}

mod http {
    pub mod request {
        use super::method::Method;
        pub struct Request {
            path: String,
            query: Option<String>,
            method: Method,
        }
    }

    pub mod method {
        pub enum Method {
            GET,
            DELETE,
            POST,
            PUT,
            HEAD,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH,
        }
    }
}
