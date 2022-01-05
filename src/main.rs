fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    // let string_slice = &string[10..];
    // let string_borrow: &str = &string;
    // let string_literal = "1234";

    // dbg!(&string);
    // dbg!(string_borrow);
    // dbg!(string_literal);

    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr: addr }
    }

    fn run(self) {
        println!("Server running on {}", self.addr)
    }
}
