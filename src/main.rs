use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

mod threadpool;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).expect("error leyendo del stream");

    println!("me llega esto: {}", String::from_utf8_lossy(&buffer[..]));

    let answer = "HTTP/1.1 200 OK\r\n\r\n";

    stream
        .write(answer.as_bytes())
        .expect("error enviando respuesta al cliente");
    stream.flush().expect("error realizando flush");
}
