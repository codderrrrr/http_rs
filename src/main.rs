#[allow(unused_imports)]
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming(){
        match stream {
            Ok(_stream) => {
                println!("New connection accepted")
            }
            Err(e) => {
                println!("Error: {}", e)
            }
        }
    }
}
