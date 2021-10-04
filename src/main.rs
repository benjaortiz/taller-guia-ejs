use threadpool::ThreadPool;

mod threadpool;
fn main() {
    let pool = ThreadPool::new(4);

    for i in 10..18 {
        pool.run(move || {
            std::thread::sleep(std::time::Duration::from_millis(250 * i));
            println!("This is Task {}", i);
        });
    }
}



/*
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
*/