/// # 参考URL。
/// - https://doc.rust-lang.org/std/net/struct.TcpStream.html |Struct std::net::TcpStream
///
/// # コマンド例。
///
/// ```
/// ### コンパイル(開発中)。
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_shogi_server
/// cargo clippy
///
/// ### コンパイル(リリース用)。
/// cargo build --release
///
/// ### 実行。
/// cargo run --release
/// 
/// ### 開けているポート確認。
/// netstat
/// ```
use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::Read;
// use std::time::Duration;
use std::{thread, time};

const CONNECTION_STRING: &str = "127.0.0.1:4091";


fn handle_client(mut stream: TcpStream) {
    println!("handle_client 0.");
    // ブロックし続けないようにする。
    stream.set_nonblocking(true).expect("set_nonblocking call failed");
    // 読み取り。
    let mut buf = vec![];
    match stream.read_to_end(&mut buf) {
        Ok(_) => {
            println!("Buf: {}", String::from_utf8_lossy(buf.as_slice()));
        },
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
            // wait until network socket is ready, typically implemented
            // via platform-specific APIs such as epoll or IOCP
            // wait_for_fd();
            println!("10 msec wait.");
            let ten_millis = time::Duration::from_millis(10);
            thread::sleep(ten_millis);
        },
        Err(e) => panic!("encountered IO error: {}", e),
    };
    println!("handle_client 1.");
}

fn main() {
    println!("I am a server!");

    // ポートを開いて待ち受け。
    let listener = TcpListener::bind(CONNECTION_STRING).unwrap();

    println!("Listen!");

    // accept connections and process them serially
    for stream in listener.incoming() {
        println!("Incoming 0.");
        handle_client(stream.unwrap());
        println!("Incoming 1.");
    }

    println!("End server.");
}
