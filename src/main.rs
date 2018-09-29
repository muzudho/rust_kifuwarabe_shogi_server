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

mod models;
use models::game_summary::*;

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

            let mut game_summary = GameSummary::new();
            game_summary.set_game_id(GAME_ID);

            res.set_message(&format!(
                r#"LOGIN:kifuwarabe OK
{}"#,
                game_summary.to_string_ln()
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
