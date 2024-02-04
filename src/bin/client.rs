use std::net::TcpStream;

fn main() {
    let addr = String::from("server:8000");
    let stream = TcpStream::connect(&addr).expect(&format!("Failed to connect to {addr}"));
}
