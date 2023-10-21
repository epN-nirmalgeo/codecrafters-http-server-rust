use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let addr = "127.0.0.1:4221";

    let listener = TcpListener::bind(&addr).await?;
    const GET: &str = "GET";
    const ECHO: &str = "/echo/";
    const USER_AGENT: &str = "/user-agent";
    const RESPONSE_OK: &[u8; 19] = b"HTTP/1.1 200 OK\r\n\r\n";
    const RESPONSE_NOT_FOUND: &[u8; 26] = b"HTTP/1.1 404 NOT FOUND\r\n\r\n";


    loop {
        let (mut socket, _)= listener.accept().await?;
        println!("socket connected");
        tokio::spawn(async move {
            println!("task spawned");
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

            println!("{:?}", lines);

            let request_type: Vec<&str> = lines[0].split(" ").collect();

            if request_type[0] == GET && request_type[1] == "/" {
                let _ = socket.write(RESPONSE_OK);
            } else if request_type[0] == GET && request_type[1].starts_with(ECHO) {
                let word = &request_type[1][6..];
                let len = word.len();
                let s = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", len, word);
                let _ = socket.write(s.as_bytes());
            } else if request_type[0] == GET && request_type[1] == USER_AGENT {
                let user_agent = &lines[2][12..];
                let len = user_agent.len();
                let s = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", len, user_agent);
                let _ = socket.write(s.as_bytes());
            }
            else {
                let _ = socket.write(RESPONSE_NOT_FOUND);
            }
        });
    }
}
