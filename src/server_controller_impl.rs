use kifuwarabe_server::interfaces::*;
use kifuwarabe_shell::shell::*;
use server_diagram_impl::*;
use shell_impl::DIAGRAM;
use std::collections::HashMap;
use std::collections::VecDeque;
use models::game::*;
use models::shell_var::*;

pub const DIAGRAM_JSON_FILE: &str = "diagram.json";

/// ロビー。マッチングするためのもの。
pub struct Lobby {
    // 余ってるプレイヤー番号。
    pub waiting_players: VecDeque<i64>,
}
impl Lobby {
    pub fn new() -> Lobby {
        Lobby {
            waiting_players: VecDeque::new(),
        }
    }
}

// グローバル変数。
use std::sync::RwLock;
lazy_static! {
    /// 待機しているプレイヤー番号など。
    pub static ref LOBBY: RwLock<Lobby> = RwLock::new(Lobby::new());

    /// クライアント（接続者番号）に対応づくオブジェクト。
    pub static ref PLAYER_MAP: RwLock<HashMap<i64,Player>> = RwLock::new(HashMap::new());    

    /// クライアントに対応づく Shell。
    pub static ref SHELL_MAP: RwLock<HashMap<i64,Shell<ShellVar>>> = RwLock::new(HashMap::new());

    /// クライアントに対応づく ShellVar。
    pub static ref SHELL_VAR_MAP: RwLock<HashMap<i64,ShellVar>> = RwLock::new(HashMap::new());

    /// 対局部屋相当。プレイヤー２つが対応付く。
    pub static ref GAME_MAP: RwLock<HashMap<i64,Game>> = RwLock::new(HashMap::new());
}

pub struct ServerController {

}
impl ServerController {
    pub fn new() -> ServerController {
        ServerController {

        }
    }
}

/// クライアントからの接続があったとき。
/// 対局待ちキューに、その接続に番号を入れる。
pub fn on_coming_shogi(connection_number: i64) {
    println!("Welcome {}!", connection_number);

    {
        // プレイヤー オブジェクトを与えようぜ☆（＾～＾）
        PLAYER_MAP
            .try_write()
            .unwrap()
            .insert(connection_number, Player::new());
    }

    {
        // シェルを与えようぜ☆（＾～＾）
        SHELL_MAP
            .try_write()
            .unwrap()
            .insert(connection_number, Shell::new());
    }

    {
        // シェルの内部状態変数を与えようぜ☆（＾～＾）
        SHELL_VAR_MAP
            .try_write()
            .unwrap()
            .insert(connection_number, ShellVar::new());
    }

    {
        // 待ち行列に追加しようぜ☆（＾ｑ＾）
        LOBBY
            .try_write()
            .unwrap()
            .waiting_players
            .push_back(connection_number);
    }
}

/// クライアントからの入力は このメソッド内で処理する。
pub fn on_receiving_shogi(req: &Request, res: &mut Response) {
    println!("<{} {}", req.get_connection_number(), req.get_message());

    match SHELL_MAP
        .try_write()
        .unwrap()
        .get_mut(&req.get_connection_number())
    {
        Some(shell) => {
            match SHELL_VAR_MAP
                .try_write()
                .unwrap()
                .get_mut(&req.get_connection_number())
            {
                Some(shell_var) => {
                    shell_var.set_connection_number(req.get_connection_number());

                    // コマンドラインは、ダイアグラムに従って パースさせるぜ☆（*＾～＾*）
                    shell_var.set_message_to_client("");
                    {
                        let mut diagram = DIAGRAM.try_write().unwrap();
                        shell.execute_line(&mut diagram, shell_var, &req.get_message());
                    }

                    let message_to_client = shell_var.get_message_to_client();

                    if &message_to_client.to_string() != "" {
                        // 応答メッセージ。
                        res.set_message(&message_to_client);
                    } else {
                        println!(
                            "<{} Nothing response message to client: [{}]",
                            req.get_connection_number(),
                            req.get_message()
                        );
                    }
                }
                None => panic!("Not found in shell var map."),
            }
        }
        None => panic!("Not found in shell map."),
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
pub fn on_sending_shogi(connection_number: i64, res: &mut Response) {
    // 2人待っていれば、マッチングしようぜ☆（＾ｑ＾）
    setup_2player_to_match();

    // クライアントが starting 状態か？
    if is_state(connection_number, "starting") {
        println!("{} は、startingだ！", connection_number);
        // 相手が CSAプロトコルと決めつけて ゲームサマリーを送り付ける。

        // クライアントが入っている部屋番号。
        let game_num = get_room_number_by_player(connection_number);
        println!("game_num: {}", game_num);

        // メッセージ作成。
        res.set_message(&get_game_summary_string(game_num));

        // ステータス変更。
        set_player_state(connection_number, "isAgree");
        println!(
            "{} のステータスを変更したはず。",
            connection_number
        );
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
