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


fn main() {
    println!("I am a server!");

    // ポートを開いて待ち受け。
    let listener = TcpListener::bind(CONNECTION_STRING).unwrap();

    println!("Listen!");

    // ずっと。接続があるたびにループが回る。
    for stream1 in listener.incoming() {
        println!("Incoming 0.");
        // 別スレッドを立てる。( FIXME スレッドを何個も立てたくないんだが)
        thread::spawn(move || {
            println!("Spawn 0.");
            handle_client(&stream1.unwrap());
            println!("Spawn 1.");
        });
        println!("Incoming 1.");
    }

    // サーバーは、[Ctrl]+[C]キーで強制終了しろだぜ☆（＾～＾）
}


/// クライアントをずっと捕まえておく。
fn handle_client(stream: &TcpStream) {
    println!("Capture client.");
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
                    print!("#Line: {}", line); // 改行は line に入っている。
                    index = 0;
                }

                // 何か応答したい。
                // LOGIN:<username> OK
                // LOGIN:incorrect
                //
                // LOGOUT:completed

                /*
BEGIN Game_Summary
Protocol_Version:1.2
Protocol_Mode:Server
Format:Shogi 1.0
Declaration:Jishogi 1.1
Game_ID:20150505-CSA25-3-5-7
Name+:TANUKI
Name-:KITSUNE
Your_Turn:+
Rematch_On_Draw:NO
To_Move:+
Max_Moves:256
BEGIN Time
Time_Unit:1sec
Total_Time:600
Byoyomi:10
Least_Time_Per_Move:1
END Time
BEGIN Position
P1-KY-KE-GI-KI-OU-KI-GI-KE-KY
P2 * -HI *  *  *  *  * -KA * 
P3-FU-FU-FU-FU-FU-FU-FU-FU-FU
P4 *  *  *  *  *  *  *  *  * 
P5 *  *  *  *  *  *  *  *  * 
P6 *  *  *  *  *  *  *  *  * 
P7+FU+FU+FU+FU+FU+FU+FU+FU+FU
P8 * +KA *  *  *  *  * +HI * 
P9+KY+KE+GI+KI+OU+KI+GI+KE+KY
P+
P-
+
+2726FU,T12
-3334FU,T6
END Position
END Game_Summary
                 */

                // REJECT:<GameID> by <rejector>
                //
                //
                // +7776FU,T12
                //
                // #SENNICHITE
                // #DRAW
                //
                // #OUTE_SENNICHITE
                // #WIN(LOSE)
                //
                // #TIME_UP
                // #WIN(LOSE)
                //
                // %TORYO,T4
                // #RESIGN
                // #WIN(LOSE)
                //
                // %KACHI,T8
                // #JISHOGI
                // #WIN(LOSE)
                //
                // %KACHI,T8
                // #ILLEGAL_MOVE
                // #WIN(LOSE)
                //
                // #MAX_MOVES
                // #CENSORED
                //
                // #CHUDAN
                //
                // またクライアントは対局中、手番にかかわらず、長さゼロの文字列、もしくはLF1文字のみをサーバに送信することができる。
                // サーバは前者を受け取った場合、単純に無視する。後者を受け取った場合、短い待ち時間の後にLF1文字のみをそのクライアントに返信する。
                // クライアントは、これらの送信を頻繁に行ってはならない。 具体的には、当該クライアントからの何らかの文字列をサーバが受信してから30秒を経ずして同一のクライアントからこれらの送信を行ってはならない。
                // クライアントがこの規定に反した場合、サーバは当該クライアントを反則負けとして扱うことができる。
                //
                // #ILLEGAL_ACTION
                // #LOSE(WIN)
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
