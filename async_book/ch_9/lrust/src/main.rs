use std::cell::RefCell;
use std::fs;
use std::time::Duration;

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::time;

use std::cmp;
use std::pin::Pin;
use std::task::Poll;

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

        tokio::spawn(handle_connection(stream));

        println!("Client connected");
    }
}

async fn handle_connection(mut stream: impl AsyncRead + AsyncWrite + Unpin) {
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
        time::sleep(Duration::from_secs(5)).await;

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

// This is mock for test http responses
struct MockTcpStream {
    read_data: Vec<u8>,
    write_data: RefCell<Vec<u8>>,
}

impl AsyncRead for MockTcpStream {
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let bufsize: usize = cmp::min(buf.remaining(), self.read_data.len());

        buf.put_slice(&self.read_data[..bufsize]);

        println!("remaining test = {}", buf.remaining());

        Poll::Ready(Ok(()))
    }
}
impl Unpin for MockTcpStream {}
impl AsyncWrite for MockTcpStream {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        let mut p = self.write_data.borrow_mut();
        *p = Vec::from(buf);

        Poll::Ready(Ok(buf.len()))
    }
    fn poll_flush(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Poll::Ready(Ok(()))
    }
    fn poll_shutdown(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Poll::Ready(Ok(()))
    }
}

// Tests our mock
#[tokio::test]
async fn test_ok() {
    let input_bytes = b"GET / HTTP/1.1\r\n";

    let mut contents = vec![0u8; 1024];
    contents[..input_bytes.len()].clone_from_slice(input_bytes);
    let mut stream = MockTcpStream {
        read_data: contents,
        write_data: RefCell::new(Vec::new()),
    };

    handle_connection(&mut stream).await;

    let expected_contents = fs::read_to_string("cat.html").unwrap();
    let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{}", expected_contents);
    assert!(stream
        .write_data
        .borrow()
        .starts_with(expected_response.as_bytes()));
}
#[tokio::test]
async fn test_not_found() {
    let input_bytes = b"GET /gwerjregjfeskwkelqwoq HTTP/1.1\r\n";

    let mut contents = vec![0u8; 1024];
    contents[..input_bytes.len()].clone_from_slice(input_bytes);
    let mut stream = MockTcpStream {
        read_data: contents,
        write_data: RefCell::new(Vec::new()),
    };

    handle_connection(&mut stream).await;

    let expected_contents = fs::read_to_string("404.html").unwrap();
    let expected_response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", expected_contents);
    assert!(stream
        .write_data
        .borrow()
        .starts_with(expected_response.as_bytes()));
}
