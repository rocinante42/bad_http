// Uncomment this block to pass the first stage
use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::{env, fs};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Error writing to buffer");
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let request = String::from_utf8_lossy(&buffer[..]);
    let lines = request.lines().map(|line| line).collect::<Vec<&str>>();
    let request_tokens: Vec<&str> = lines[0].split(" ").collect();
    let user_agent: Vec<&str> = lines[2].split(" ").collect();
    let mut response = "HTTP/1.1 400 Bad Request\r\n\r\n".to_string();

    match request_tokens[0] {
        "GET" => {
            if request_tokens[1] == "/" {
                response = "HTTP/1.1 200 OK\r\n\r\n".to_string()
            } else if request_tokens[1].starts_with("/echo/") {
                let message = request_tokens[1].replace("/echo/", "");
                let len = message.len();
                response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    len, message
                );
                println!("{}", response);
            } else if request_tokens[1].starts_with("/user-agent") {
                let message = user_agent[1];
                let len = message.len();
                response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    len, message
                )
            } else if request_tokens[1].starts_with("/files/") {
                let filename = request_tokens[1].replace("/files/", "");
                let env_args: Vec<String> = env::args().collect();
                let mut dir = env_args[2].clone();
                dir.push_str(&filename);
                let file = fs::read(dir);

                match file {
                    Ok(fc) => {
                        response =  format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}\r\n", fc.len(), String::from_utf8(fc).expect("file content"));
                    }
                    Err(_) => {
                        response = "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
                    }
                }
            } else {
                response = "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
            }
        }
        _ => {}
    }
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                std::thread::spawn(|| handle_connection(stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
