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
use std::io::Read;

const CONNECTION_STRING: &str = "127.0.0.1:4091";


fn handle_client(mut stream: TcpStream) {
    let mut buf = vec![];
    match stream.read_to_end(&mut buf) {
        Ok(_) => {
            println!("Buf: {}", String::from_utf8_lossy(buf.as_slice()));
        }
        Err(e) => panic!("encountered IO error: {}", e),
    };
}

fn main() {
    println!("I am a server!");

    // ポートを開いて待ち受け。
    let listener = TcpListener::bind(CONNECTION_STRING).unwrap();

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }

    println!("End server.");
}
