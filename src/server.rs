use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr: addr }
    }

    pub fn run(self) {
        println!("Server running on http://{}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            // let res = listener.accept();
            // let (stream, addr) = res.unwrap();
            match listener.accept() {
                Ok((stream, _)) => {
                    println!("Ok");
                }
                Err(er) => println!("Error {}", er),
            }
        }
    }
}
