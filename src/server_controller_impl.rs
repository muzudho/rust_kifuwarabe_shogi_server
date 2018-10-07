use kifuwarabe_server::interfaces::*;
use shell_impl::*;
use server_diagram_impl::*;
use std::collections::VecDeque;

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
    /// サーバーのどこからでも使う。
    pub static ref LOBBY: RwLock<Lobby> = RwLock::new(Lobby::new());
}


/// クライアントからの接続があったとき。
/// 対局待ちキューに、その接続に番号を入れる。
pub fn on_coming_shogi(connection_number: i64) {
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

/// クライアントからの入力は このメソッド内で処理する。
pub fn on_receiving_shogi(req: &Request, res: &mut Response) {
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
        println!("{} のステータスを変更したはず。", connection_number);
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
