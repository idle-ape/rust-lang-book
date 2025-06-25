use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_server::ThreadPool;

fn main() {
    let ln = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in ln.incoming() {
        let conn = stream.unwrap();
        println!(
            "Connection established!, remote addr: {}",
            conn.peer_addr().unwrap()
        );
        pool.execute(|| handle_conn(conn));
    }
}

fn handle_conn(mut stream: TcpStream) {
    let mut buff = [0; 512];
    stream.read(&mut buff).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buff));

    let get = b"GET / HTTP/1.1\r\n";
    // 模拟慢请求
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buff.starts_with(get) {
        (
            "HTTP/1.1 200 OK\r\n\r\n",
            "/data/rust-lang-book/ch20/web-server/src/hello.html",
        )
    } else if buff.starts_with(sleep) {
        thread::sleep(Duration::from_secs(20));
        (
            "HTTP/1.1 200 OK\r\n\r\n",
            "/data/rust-lang-book/ch20/web-server/src/hello.html",
        )
    } else {
        (
            "HTTP/1.1 404 NOT FOUND\r\n\r\n",
            "/data/rust-lang-book/ch20/web-server/src/404.html",
        )
    };

    let content = fs::read_to_string(filename).unwrap();

    let resp = format!("{}{}", status_line, content);
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}
