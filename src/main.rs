use std::net::TcpListener;
use std::thread;

use handlers::handle_connection;

mod handlers;
mod request;
mod response;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("error: {}", e),
            Ok(mut stream) => {
                thread::spawn(move || handle_connection(&mut stream));
            }
        }
    }
}
