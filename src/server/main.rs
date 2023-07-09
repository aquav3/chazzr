use std::net::{TcpListener, TcpStream, SocketAddr};

fn bind_to_ip(ip: &str) -> Option<TcpListener> {
    if let Ok(ls) = TcpListener::bind(ip) {
        println!("Bound to ip address succesfully!");

        return Some(ls)

    }
    println!("Failed to bind to ip address!");
    return None;
}

fn handle_stream(stream_data: (TcpStream, SocketAddr)) {
    println!("Accepted connection: {:?}", stream_data.1);
    loop {}    
}
     

fn main() {
    if let Some(listener) = bind_to_ip("127.0.0.1:8080") {
        if let Ok(stream) = listener.accept() {
            handle_stream(stream)
        }           
    }   
    else {
        println!("Exiting the program due to the error!");
    }
}
