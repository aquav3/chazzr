use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

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
}
     

fn main() {
    if let Some(listener) = bind_to_ip("127.0.0.1:8080") {
        for stream in listener.incoming() {
            if stream.is_ok() {
                handle_stream(stream.unwrap())
            }
        }
    }   
    else {
        println!("Exiting the program due to the error!");
    }
}
