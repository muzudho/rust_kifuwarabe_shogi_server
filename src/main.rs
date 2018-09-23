/// # 参考URL。
/// - https://doc.rust-lang.org/std/net/struct.TcpStream.html |Struct std::net::TcpStream
/// - https://gigazine.net/news/20120831-10000-jointer-1-server-cedec2012/ |サーバーマシン1台で同時接続者数1万名を実現するにはどうすればいいのかというノウハウと考え方
///
/// # コマンド例。
///
/// ```
/// ### コンパイル(開発中)。 [Windows]+[R]キー, "cmd"+[Enter]。
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
use std::{thread, time};
use std::collections::HashMap;
#[macro_use]
extern crate lazy_static;
use std::time::Duration;


const CONNECTION_STRING: &str = "127.0.0.1:4081";

/// クライアントの変数。
struct ClientVar {

}
impl ClientVar {
    pub fn new() -> ClientVar {
        ClientVar {

        }
    }
}

// グローバル変数。
use std::sync::RwLock;
lazy_static! {
    /// スレッド間で共有する。 <接続番号,変数>
    static ref CLIENT_MAP: RwLock<HashMap<i64, ClientVar>> = RwLock::new(HashMap::new());
}


fn main() {
    println!("I am a server!");

    // 接続受付スレッド。
    thread::spawn(move || {
        println!("Listen!");
        let mut connection_number = 0;
        let sever_listener = TcpListener::bind(CONNECTION_STRING).unwrap();
        for stream_wrap in sever_listener.incoming() {
            CLIENT_MAP.try_write().unwrap().insert(connection_number, ClientVar::new());
            thread::spawn(move || {
                println!("Welcome!");
                handle_client(connection_number, &stream_wrap.unwrap());
            });
            connection_number += 1;
        }
    });

    // 各クライアントに何かしたいことがあれば 以下に書く。
    loop {
        /*
        let mut count = 0;
        match CLIENT_MAP.try_read() {
            Ok(client_map) => {
                for (_connection_number, _client_var) in client_map.iter() {
                    count += 1;
                }
            },
            Err(_) => unreachable!(),
        };
        println!("count = {}", count);
        */
        thread::sleep(Duration::from_millis(1));
    }
    // サーバーは、[Ctrl]+[C]キーで強制終了しろだぜ☆（＾～＾）
}


/// クライアントをずっと捕まえておく。
fn handle_client(connection_number:i64, stream: &TcpStream) {
    println!("Capture client. {}", connection_number);

    // TODO クライアント名を取得したい。
    // let name = "Kifuwarabe"; // 仮

    // ブロックし続けないようにする。
    // let _ = stream.set_read_timeout(Some(Duration::new(10, 0)));
    stream.set_nonblocking(true).expect("set_nonblocking call failed");
    let mut buf = [0];
    let mut buf_arr = [0; 1024];
    let mut index = 0;
    // FIXME 切断のエラーをキャッチしたい。
    loop {
        // 読み取り。
        // FIXME マルチバイト文字の受け取り方が分からん☆（＾～＾）1バイトずつ取る。
        match stream.take(1).read(&mut buf) {
            Ok(len) => {
                if 0==len {
                    // 長さが 0 なら切断と判定する。
                    break;
                }
                buf_arr[index] = buf[0];
                index += 1;
                if buf[0] == b'\n' {
                    let line = String::from_utf8_lossy(&buf_arr[0..index]);
                    print!("{}> {}", connection_number, line); // 改行は line に入っている。
                    index = 0;
                }

                // 何か応答したい。
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // ブロックしなければ、ここにくる。
                // print!("[wait]");
                let msec = time::Duration::from_millis(10);
                thread::sleep(msec);
            },
            Err(e) => panic!("encountered IO error: {}", e),
        };
    }
    println!("Release client.");
}
