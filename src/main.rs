use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}, fs::File};
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let args: Vec<String> = env::args().collect();
    let mut directory = Arc::new("".to_owned());
    if args.len() >= 3 {
        directory =Arc::new(args[2].clone());   
    }
    let addr = "127.0.0.1:4221";

    let listener = TcpListener::bind(&addr).await?;
    const GET: &str = "GET";
    const POST: &str = "POST";
    const ECHO: &str = "/echo/";
    const FILE: &str = "/files/";
    const USER_AGENT: &str = "/user-agent";
    const RESPONSE_OK: &[u8; 19] = b"HTTP/1.1 200 OK\r\n\r\n";
    const RESPONSE_POST_OK: &[u8; 19] = b"HTTP/1.1 201 OK\r\n\r\n";
    const RESPONSE_NOT_FOUND: &[u8; 26] = b"HTTP/1.1 404 NOT FOUND\r\n\r\n";


    loop {
        let (mut socket, _)= listener.accept().await?;
        let dir = Arc::clone(&directory);
        tokio::spawn(async move {
            let mut data = [0u8; 4096];
            let bytes_read = socket
                .read(&mut data)
                .await
                .expect("failed to read data from socket");

            if bytes_read == 0 {
                return ;
            }
            let request = String::from_utf8_lossy(&data);
            let lines: Vec<&str> = request.lines().collect();


            let request_type: Vec<&str> = lines[0].split(" ").collect();
            if request_type[0] == GET && request_type[1] == "/" {
                let _ = socket.write(RESPONSE_OK).await;
            } else if request_type[0] == GET && request_type[1].starts_with(ECHO) {
                let word = &request_type[1][6..];
                let len = word.len();
                let s = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", len, word);
                let _ = socket.write(s.as_bytes()).await;
            } else if request_type[0] == GET && request_type[1] == USER_AGENT {
                let user_agent = &lines[2][12..];
                let len = user_agent.len();
                let s = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", len, user_agent);
                let _ = socket.write(s.as_bytes()).await;
            } else if request_type[0] == GET && request_type[1].starts_with(FILE) {
                let mut file_name = dir.clone().to_string();
                file_name.push_str(&request_type[1][7..]);
                match File::open(file_name).await {
                    Ok(mut file) => {
                        let mut contents = vec![];
                        file.read_to_end(&mut contents).await.unwrap();
                        let s = format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n", contents.len());
                        let _= socket.write(s.as_bytes()).await;
                        let _ = socket.write_all(&contents).await;
                    }
                    Err(_) => {
                        let _ = socket.write(RESPONSE_NOT_FOUND).await;
                    }
                }
            } else if request_type[0] == POST && request_type[1].starts_with(FILE) {
                let mut file_name = dir.clone().to_string();
                file_name.push_str(&request_type[1][7..]);
                let content_length = lines[3][16..].to_string().parse::<usize>().unwrap();
                println!("content length {}", content_length);

                match File::create(file_name).await {
                    Ok(mut file) => {
                        let content = lines[6][..=content_length].to_string();
                        println!(" content : {content} {}", content.len());
                        if let Ok(_) = file.write(content.as_bytes()).await {
                            let _ = socket.write(RESPONSE_POST_OK).await;
                        } else {
                            let _ = socket.write(RESPONSE_NOT_FOUND).await;
                        }
                    }
                    Err(_) => {
                        let _ = socket.write(RESPONSE_NOT_FOUND).await;
                    }
                }
                
            } else {
                let _ = socket.write(RESPONSE_NOT_FOUND).await;
            }
        });
    }
}
