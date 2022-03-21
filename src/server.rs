use std::{net::TcpListener};
use std::io::{Read, Write};


pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self) {        
        println!("Linstenning on {} ...", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        
        loop {

            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("OK");
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_ ) =>{
                            println!("{}",String::from_utf8_lossy(&buffer));
                        },
                        Err(e) => println!("Failed Stream {}.",e)
                    }
                }, 
                Err(err) => {
                    print!("Fail to estabilish connection: {}", err);
                }
            }


            let res = listener.accept();
            if res.is_err(){
                continue;
            }
            
            let (stream, addr) = res.unwrap();


        }
        
    }
}
