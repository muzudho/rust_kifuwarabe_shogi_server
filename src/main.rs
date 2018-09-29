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
use std::collections::HashMap;

extern crate kifuwarabe_server;
use kifuwarabe_server::interfaces::*;
use kifuwarabe_server::*;

mod models;
use models::game_summary::*;

extern crate kifuwarabe_shell;
extern crate serde_json;

mod shell_impl;
use shell_impl::*;

const CONNECTION_STRING: &str = "127.0.0.1:4081";

/// 対局変数。
struct Game {
    /// 汎用的に利用できるハッシュマップ。
    #[allow(dead_code)]
    properties: HashMap<String, String>,
    game_summary: GameSummary,
}
impl Game {
    pub fn new() -> Game {
        Game {
            properties: HashMap::new(),
            game_summary: GameSummary::new(),
        }
    }
}

// グローバル変数。
use std::sync::RwLock;
lazy_static! {
    /// 対局間で共有する。 <対局番号,変数>
    static ref GAME_MAP: RwLock<HashMap<i64, Game>> = RwLock::new(HashMap::new());
}

const GAME_ID: &str = "20180929-KIFUWARABECUP-0";

fn main() {
    println!("I am a shogi server!");

    // グローバル変数の用意。
    setup_graph();

    {
        let mut game = Game::new();

        // とりあえず、対局室を１つだけ作成。
        game.game_summary.set_game_id(GAME_ID);
        game.game_summary
            .set_name_arr(["".to_string(), "".to_string()]);
        game.game_summary.set_turn(Turn::Black);
        GAME_MAP.try_write().unwrap().insert(0, game);
    }

    // サーバー構造体に、コールバック関数を登録していけだぜ。
    let server = &Server {
        coming: default_coming,
        receiving: default_receiving,
    };

    listen(&server, CONNECTION_STRING);
    // サーバーは、[Ctrl]+[C]キーで強制終了しろだぜ☆（＾～＾）
}

/**
 * クライアントからの接続があったとき、その接続に番号を振る。
 */
fn default_coming(connection_number: i64) {
    println!("Welcome {}!", connection_number);

    // 接続番号をたよりに ここで変数を初期化したりする。
}

/**
 * クライアントからの入力は このメソッド内で処理する。
 */
fn default_receiving(req: &Request, res: &mut Response) {
    println!("<{} {}", req.get_connection_number(), req.get_message());

    match req.get_message() {
        "LOGIN kifuwarabe a" => {
            // 名前とパスワードを分解したい。
            execute_line(req.get_connection_number(), &req.get_message().to_string());

            res.set_message(&format!(
                r#"LOGIN:kifuwarabe OK
{}"#,
                GAME_MAP.try_read().unwrap()[&0].game_summary.to_string_ln()
            ))
        }
        r#"
AGREE"# => {
            res.set_message(&format!(
                r#"START:{}
"#, // 最後に改行が必要。
                GAME_MAP.try_read().unwrap()[&0].game_summary.get_game_id()
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
