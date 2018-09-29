/// # 参考URL。
/// - https://doc.rust-lang.org/std/net/struct.TcpStream.html |Struct std::net::TcpStream
/// - https://gigazine.net/news/20120831-10000-jointer-1-server-cedec2012/ |サーバーマシン1台で同時接続者数1万名を実現するにはどうすればいいのかというノウハウと考え方
///
/// # コマンド例。
///
/// ```
/// ###  [Windows]+[R]キー, "cmd"+[Enter]。
/// cls
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

#[macro_use]
extern crate lazy_static;
//use std::collections::HashMap;
//use std::collections::VecDeque;

extern crate kifuwarabe_server;
use kifuwarabe_server::interfaces::*;
use kifuwarabe_server::*;

mod models;
//use models::game_summary::*;

extern crate kifuwarabe_shell;
extern crate serde_json;

mod shell_impl;
use shell_impl::*;

mod server_impl;
use server_impl::*;
use server_impl::LOBBY;
//use server_impl::GAME_MAP;

use std::thread;
use std::time::Duration;

const CONNECTION_STRING: &str = "127.0.0.1:4081";

fn main() {
    println!("I am a shogi server!");

    // グローバル変数の用意。
    setup_graph();

    // サーバー構造体に、コールバック関数を登録していけだぜ。
    let server = &Server {
        on_coming: default_coming,
        on_receiving: default_receiving,
        on_sending: default_sending,
    };

    // 静的に、受信ポートを開いて待機。
    listen_incoming(&server, CONNECTION_STRING);

    loop {
        thread::sleep(Duration::from_millis(1));
    }
    // サーバーは、[Ctrl]+[C]キーで強制終了しろだぜ☆（＾～＾）
}

/**
 * クライアントからの接続があったとき、その接続に番号を振る。
 */
fn default_coming(connection_number: i64) {
    println!("Welcome {}!", connection_number);

    {
        // 待ち行列に追加しようぜ☆（＾ｑ＾）
        LOBBY
            .try_write()
            .unwrap()
            .waiting_players
            .push_back(connection_number);
    }
}

/**
 * クライアントからの入力は このメソッド内で処理する。
 */
fn default_receiving(req: &Request, res: &mut Response) {
    println!("<{} {}", req.get_connection_number(), req.get_message());

    // パースは丸投げ☆（*＾～＾*）
    let flow_message = execute_line_by_client(req.get_connection_number(), &req.get_message());

    if &flow_message.to_string() == "loginEnd" {
        // 名前とパスワードを分解した。

        // 応答メッセージ作成。
        res.set_message(&format!(
            r#"LOGIN:{} OK
"#, // 改行。
            get_player_name(req.get_connection_number())
        ));

    } else {
        println!(
            "<{} Not match: [{}]",
            req.get_connection_number(),
            req.get_message()
        );
    }

/* TODO
    match req.get_message() {
        "LOGIN kifuwarabe a" => {

        }
        r#"
AGREE"# => {
            res.set_message(&format!(
                r#"START:{}
"#, // 最後に改行が必要。
                GAME_MAP.try_read().unwrap()[&0].game_summary.get_game_id()
            ));
        }
    }
*/
}

/**
 * クライアントのいずれか１つが、サーバーからのメッセージを待っているタイミング。
 */
fn default_sending(connection_number: i64, _res: &mut Response) {

    // 2人待っていれば、マッチングしようぜ☆（＾ｑ＾）
    setup_2player_to_match();
    
        /*

    if is_state(connection_number, "starting") {

    }

            // メッセージ作成。
            /* TODO             get_game_summary_string()
            res.set_message(&format!(
                r#"{}"#,
                GAME_MAP.try_read().unwrap()[&0].game_summary.to_string_ln()
            ))
             */
        */

    /*
    let waiting_player;
    if -1!=waiting_player {
        // 誰かが待機中。
        // マッチングが成立。
        println!("Match {} vs {}!", connection_number, waiting_player);
    } else {
        // 自分が待機になる。
        LOBBY.try_write().unwrap().waiting_player = connection_number;
    }
     */
}
