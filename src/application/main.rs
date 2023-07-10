use std::net::*;
use std::io::{Write, Read};
use serde::*;

#[derive(Default, Serialize, Deserialize)]
struct UserData {
    name: String,
    message: String
}


fn connect_to_ip(ip: &str) -> Option<TcpStream> {
    match TcpStream::connect(ip) {
        Ok(stream) => return Some(stream),
        Err(_) => return None,
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut user_data = UserData::default();

    println!("Enter your username.");
    std::io::stdin().read_line(&mut user_data.name).expect("Failed to read input!");
    user_data.name = user_data.name.trim().to_string();

    'running: loop {
        user_data.message = "".to_string();
        std::io::stdin().read_line(&mut user_data.message).expect("Failed to read input!");
        user_data.message = user_data.message.trim().to_string();

        if user_data.message == "exit" {
            break 'running;
        }
        let data = serde_json::to_string(&user_data).unwrap();
        println!("{}", data);
        stream.write(data.as_bytes()).unwrap();
            
    }
}

fn main() {
    if let Some(stream) = connect_to_ip("127.0.0.1:8080") {
        println!("Connected to server!");
        handle_connection(stream);
    }  
    else {
        println!("Failed to connect to server!");
    }
}
