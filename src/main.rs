use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        thread::spawn(|| match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => println!("error: {}", e),
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line: Vec<&str> = http_request[0].as_str().split(" ").collect();
    let headers = parse_headers(&http_request);
    let route = request_line[1];
    let response = match route {
        "/" => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
        route if route.starts_with("/echo/") => handle_echo(route.to_string()),
        "/user-agent" => handle_user_agent(&headers),
        _ => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
    };

    stream.write_all(response.as_bytes()).unwrap();
}

fn parse_headers(http_request: &Vec<String>) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    for i in 1..http_request.len() {
        if http_request[i] != "" {
            let entry: Vec<&str> = http_request[i].split(": ").collect();
            headers.insert(entry[0].into(), entry[1].into());
        }
    }
    headers
}

fn handle_echo(route: String) -> String {
    let route = route.split("/").collect::<Vec<&str>>();
    let content = route.last().unwrap();
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        content.chars().count(),
        content
    )
}

fn handle_user_agent(headers: &HashMap<String, String>) -> String {
    let content = headers.get("User-Agent").unwrap();
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        content.chars().count(),
        content
    )
}
