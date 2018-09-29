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
use std::collections::VecDeque;

extern crate kifuwarabe_server;
use kifuwarabe_server::interfaces::*;
use kifuwarabe_server::*;

mod models;
use models::game_summary::*;

extern crate kifuwarabe_shell;
extern crate serde_json;

mod shell_impl;
use shell_impl::*;

mod server_impl;
use server_impl::*;

use std::thread;
use std::time::Duration;

const CONNECTION_STRING: &str = "127.0.0.1:4081";

/// ロビー。マッチングするためのもの。
struct Lobby {
    // 余ってるプレイヤー番号。
    waiting_players: VecDeque<i64>,
}
impl Lobby {
    pub fn new() -> Lobby {
        Lobby {
            waiting_players: VecDeque::new(),
        }
    }
}

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
    /// サーバーのどこからでも使う。
    static ref LOBBY: RwLock<Lobby> = RwLock::new(Lobby::new());

    /// 対局間で共有する。 <対局番号,変数>
    static ref GAME_MAP: RwLock<HashMap<i64, Game>> = RwLock::new(HashMap::new());
}

const GAME_ID: &str = "20180929-KIFUWARABECUP-0";

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

    match req.get_message() {
        "LOGIN kifuwarabe a" => {
            // 名前とパスワードを分解したい。
            execute_line(req.get_connection_number(), &req.get_message().to_string());
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

/**
 * サーバーからクライアントへメッセージを送信できるタイミング。
 */
fn default_sending(_connection_number: i64, _res: &mut Response) {
    // 2人待っていれば、マッチングしようぜ☆（＾ｑ＾）
    if 1 < LOBBY.try_read().unwrap().waiting_players.len() {
        let player_num0 = LOBBY
            .try_write()
            .unwrap()
            .waiting_players
            .pop_front()
            .unwrap();
        let player_num1 = LOBBY
            .try_write()
            .unwrap()
            .waiting_players
            .pop_front()
            .unwrap();
        println!("マッチング {} vs {}.", player_num0, player_num1);

        // 対局室を作成。
        let game_number = create_game(player_num0, player_num1);
        println!(
            "対局室 {} を作成。 {} vs {}.",
            game_number, player_num0, player_num1
        );

        println!("プレイヤー0: {} を ゲームルーム {} へ移動。",
            player_num0, game_number);
        set_player_to_game_room(player_num0, game_number as i64);
        set_player_state(player_num0, "starting");

        println!("プレイヤー1: {} を ゲームルーム {} へ移動。",
            player_num1, game_number);
        set_player_to_game_room(player_num1, game_number as i64);
        set_player_state(player_num1, "starting");

        println!("マッチング終わり。");
    }
    {
        /*

            // メッセージ作成。
            /*
            res.set_message(&format!(
                r#"LOGIN:kifuwarabe OK
{}"#,
                GAME_MAP.try_read().unwrap()[&0].game_summary.to_string_ln()
            ))
             */
        */
    }

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

/**
 * 対局室を１つ作成。
 *
 * # Returns.
 * ゲーム部屋番号。
 */
fn create_game(player_num0:i64, player_num1:i64) -> usize {
    let game_number;

    println!("プレイヤーデータ0: {} 取得", player_num0);
    let player_name0 = get_player_data(player_num0);

    println!("プレイヤーデータ1: {} 取得", player_num1);
    let player_name1 = get_player_data(player_num1);

    println!("対局室オブジェクト生成");
    let mut game = Game::new();

    {

        // とりあえず、
        println!("ゲームIDセット");
        game.game_summary.set_game_id(GAME_ID);

        println!("名前配列セット");
        game.game_summary
            .set_name_arr([player_name0.to_string(), player_name1.to_string()]);

        println!("ターン セット");
        game.game_summary.set_turn(Turn::Black);

        {
            println!("部屋番号取得");
            game_number = GAME_MAP.try_write().unwrap().len();
            println!("部屋番号 {}", game_number);
        }
        {
            println!("部屋挿入");
            GAME_MAP
                .try_write()
                .unwrap()
                .insert(game_number as i64, game);
        }
    }

    game_number
}
