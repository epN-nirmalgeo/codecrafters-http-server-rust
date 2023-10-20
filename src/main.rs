use std::{net::TcpListener, io::Write};

fn main() {

    let listener = TcpListener::bind("localhost:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let _ = _stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n");
            } 
            Err(e) => {
                println!("error : {}", e);
            }
        }
    }
}
