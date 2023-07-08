use std::net::TcpListener;

fn main() {
    let mut listener = TcpListener::bind("127.0.0.1:80");

    if let Ok(ls) = listener {
        listener = Ok(ls);
        println!("Bound to ip address succesfully!");
    }
    else {
        println!("Failed to bind to ip address!");
        return
    }
     
}
