use std::net::*;

fn connect_to_ip(ip: &str) -> Option<TcpStream> {
    match TcpStream::connect(ip) {
        Ok(stream) => return Some(stream),
        Err(_) => return None,
    }
}

fn main() {
    if let Some(stream) = connect_to_ip("127.0.0.1:8080") {
        println!("Connected to server!");
    }  
    else {
        println!("Failed to connect to server!");
    }
}
