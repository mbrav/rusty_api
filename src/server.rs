use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr: addr }
    }

    pub fn run(self) {
        println!("Server running on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {}
    }
}
