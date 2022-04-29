use std::net::TcpListener;
mod send;
use send::handle_connection;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Listening on- http://127.0.0.1:7878/");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
    
}