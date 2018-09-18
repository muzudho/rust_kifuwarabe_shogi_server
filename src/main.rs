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

// クライアントをずっと捕まえておく。
fn handle_client(mut stream: &TcpStream) {
    println!("Capture client.");
    loop {
        // ブロックし続けないようにする。
        // stream.set_nonblocking(true).expect("set_nonblocking call failed");
        // 読み取り。
        let mut buf = vec![];
        match stream.read_to_end(&mut buf) {
            Ok(len) => {
                println!("Buf: {}, Len: {}.", String::from_utf8_lossy(buf.as_slice()), len);
                if 0==len {
                    // 長さが 0 なら切断と判定する。
                    break;
                }
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
    }
    println!("Release client.");
}

fn main() {
    println!("I am a server!");

    // ポートを開いて待ち受け。
    let listener = TcpListener::bind(CONNECTION_STRING).unwrap();

    println!("Listen!");

    // ずっと。接続があるたびにループが回る。
    for stream1 in listener.incoming() {
        println!("Incoming 0.");
        // 別スレッドを立てる。
        thread::spawn(move || {
            println!("Spawn 0.");
            handle_client(&stream1.unwrap());
            println!("Spawn 1.");
        });
        println!("Incoming 1.");
    }

    // println!("End server.");
}
