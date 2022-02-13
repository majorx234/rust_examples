use async_std::fs;
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::prelude::*;
use futures::executor::block_on;

async fn handle_connection(mut stream: TcpStream) {
    let get = b"GET / HTTP/1.1\r\n";
    // need Bufer for first 1024bytes
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    // depending on the message send Hello World or 404 error
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).await.unwrap();

    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

async fn http_listen() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    let mut incoming = listener.incoming();

    // wait for incomming connections
    while let Some(tcpstream) = incoming.next().await {
        handle_connection(tcpstream.unwrap()).await;
    }
}

fn main() {
    block_on(http_listen())
}
