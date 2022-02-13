use futures::executor::block_on;

async fn handle_connection(mut stream: TcpStream) {
    //...
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
