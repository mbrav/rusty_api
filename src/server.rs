use std::io::Read;
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
                Ok((mut stream, _)) => {
                    println!("Ok");
                    // 1 KB Buffer
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received: {}", String::from_utf8_lossy(&buffer))
                        }
                        Err(er) => println!("Connection error {}", er),
                    }
                }
                Err(er) => println!("Error {}", er),
            }
        }
    }
}
