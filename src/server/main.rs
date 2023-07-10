use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
struct UserData {
    name: String,
    message: String
}

fn bind_to_ip(ip: &str) -> Option<TcpListener> {
    if let Ok(ls) = TcpListener::bind(ip) {
        println!("Bound to ip address succesfully!");

        return Some(ls)

    }
    println!("Failed to bind to ip address!");
    return None;
}

fn handle_stream(mut stream: TcpStream) {
    println!("Accepted connection");

    loop {
        let mut buff = [0; 1024];
        let bytes_read = match stream.read(&mut buff) {
            Ok(n) if n > 0 => n,
            _ => break,
        };
        let message = String::from_utf8_lossy(&buff[..bytes_read]);
        println!("{}", message);
    }
}
     

fn main() {
    if let Some(listener) = bind_to_ip("127.0.0.1:8080") {
        for stream in listener.incoming() {
            if stream.is_ok() {
                thread::spawn(move || handle_stream(stream.unwrap()));  
            }
        }
    }   
    else {
        println!("Exiting the program due to the error!");
    }
}
