use std::{net::TcpListener, io::{Write, Read}};

fn main() {

    let listener = TcpListener::bind("localhost:4221").unwrap();
    const GET: &str = "GET";
    const ECHO: &str = "/echo/";
    const RESPONSE_NOT_FOUND: &[u8; 26] = b"HTTP/1.1 404 NOT FOUND\r\n\r\n";

    for stream_result in listener.incoming() {
        match stream_result {
            Ok(mut stream) => {
                let mut data = [0u8; 4096];
                stream.read(&mut data).unwrap();
                let request = String::from_utf8_lossy(&data);
                let lines: Vec<&str>= request.lines().collect();
                let line_token: Vec<&str> = lines[0].split(' ').collect();

                if line_token[0] == GET && line_token[1].starts_with(ECHO) {
                    let word = &line_token[1][6..];
                    let len = word.len();
                    let s = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n", len, word);
                    let _ = stream.write_all(s.as_bytes());
                } else {
                    let _ = stream.write_all(RESPONSE_NOT_FOUND);
                }

            } 
            Err(e) => {
                println!("error : {}", e);
            }
        }
    }
}
