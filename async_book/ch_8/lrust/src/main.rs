use lib;
use std::fs;

use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

use tokio::net::TcpListener;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    // press while holding 'Ctrl' to:
    //                       quickly open site: >> http://127.0.0.1:7878/ <<
    //                  see 5 sec server sleep: >> http://127.0.0.1:7878/sleep; <<
    //                   see error `NOT FOUND`: >> http://127.0.0.1:7878/gjksjaqwoep <<

    // Listen incoming TCP connections to localhost 6785
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    println!("Server started");

    loop {
        // Accept incomming connection; second field
        // ignored, because we don't need use socket addres
        let (stream, _) = listener.accept().await.unwrap();

        println!("Client connected");

        handle_connection(stream).await;
    }
}

async fn handle_connection(mut stream: TcpStream) {
    // Read the first 1024 bytes of data from the stream
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep_get = b"GET /sleep HTTP/1.1\r\n";

    // Respond with ok, sleep and
    // depending on the data in the request
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "cat.html")
    } else if buffer.starts_with(sleep_get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "cat.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    // Write response back to the stream,
    // and flush the stream to ensure the response is sent back to the client
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
