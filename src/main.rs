use std::net::TcpListener;

fn main() {

    let listener = TcpListener::bind("localhost:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_) => {
                println!("Accept incoming connections");
            } 
            Err(e) => {
                println!("error : {}", e);
            }
        }
    }
}
