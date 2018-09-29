/// # 参考URL。
/// - https://doc.rust-lang.org/std/net/struct.TcpStream.html |Struct std::net::TcpStream
/// - https://gigazine.net/news/20120831-10000-jointer-1-server-cedec2012/ |サーバーマシン1台で同時接続者数1万名を実現するにはどうすればいいのかというノウハウと考え方
///
/// # コマンド例。
///
/// ```
/// ###  [Windows]+[R]キー, "cmd"+[Enter]。
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_shogi_server
///
/// ### コンパイル(開発中)。
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
extern crate lazy_static;

extern crate kifuwarabe_server;
use kifuwarabe_server::interfaces::*;
use kifuwarabe_server::*;

const CONNECTION_STRING: &str = "127.0.0.1:4081";

fn main() {
    println!("I am a shogi server!");

    let server = &Server {
        receiver: default_receiver,
    };

    listen(&server, CONNECTION_STRING);
    // サーバーは、[Ctrl]+[C]キーで強制終了しろだぜ☆（＾～＾）
}

const GAME_ID: &str = "20180929-KFWARABE1-10203";

/**
 * クライアントからの入力は このメソッド内で処理する。
 */
fn default_receiver(req: &Request, res: &mut Response) {
    println!("<{} {}", req.get_connection_number(), req.get_message());

    match req.get_message() {
        "LOGIN kifuwarabe a" => {
            res.set_message(&format!(
                r#"LOGIN:kifuwarabe OK
BEGIN Game_Summary
Protocol_Version:1.2
Protocol_Mode:Server
Format:Shogi 1.0
Declaration:Jishogi 1.1
Game_ID:{}
Name+:kifuwarabe
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
END Position
END Game_Summary
"#, // 最後に改行が必要。
                GAME_ID
            ))
        }
        r#"
AGREE"# => {
            res.set_message(&format!(
                r#"START:{}
"#, // 最後に改行が必要。
                GAME_ID
            ));
        }
        _ => {
            println!(
                "<{} Not match: [{}]",
                req.get_connection_number(),
                req.get_message()
            );
        }
    }
}
