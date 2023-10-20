use std::{net::TcpListener, io::{Write, Read}};

fn main() {

    let listener = TcpListener::bind("localhost:4221").unwrap();
    const GET_REQUEST: &str= "GET / HTTP/1.1";

    for stream_result in listener.incoming() {
        match stream_result {
            Ok(mut stream) => {
                let mut data = [0u8; 4096];
                stream.read(&mut data).unwrap();
                let request = String::from_utf8_lossy(&data);
                for line in request.lines() {
                    println!("{line}");
                    if line == GET_REQUEST {
                        let _ = stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n");
                    } else {
                        let _ = stream.write_all(b"HTTP/1.1 404 NOT FOUND\r\n\r\n");
                    }
                    break;
                }


            } 
            Err(e) => {
                println!("error : {}", e);
            }
        }
    }
}
