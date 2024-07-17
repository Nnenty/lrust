use lib::ThreadPool;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

fn main() {
    // press while holding 'Ctrl' to quickly open site: http://127.0.0.1:7878/
    //                       to see 5 sec server sleep: http://127.0.0.1:7878/sleep
    //                          to see error `NOT FOUND`: http://127.0.0.1:7878/gjksjaqwoep

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    println!("Sever started.");

    // server accepts only 3 requests, after which it stops working
    for request in listener.incoming().take(3) {
        let request = request.unwrap();

        println!("Сonnection established");
        pool.execute(|| {
            handle_connection(request);
        });
    }
}
fn handle_connection(mut request: TcpStream) {
    let buf_reader = BufReader::new(&request);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status, html_str): (&str, &str) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }

        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let resp = fs::read_to_string(html_str).expect("");
    let resp_len = resp.len();

    let response = format!(
        "{status}\
        Content-Length: {resp_len}\r\n\r\n\
        {resp}"
    );

    request
        .write_all(response.as_bytes())
        .expect("err to send response");

    // info about request:
    // println!("{request_stream:#?}");
}
