use kifuwarabe_server::*;
use models::game_summary::*;
use std::collections::HashMap;
use std::collections::VecDeque;

const GAME_ID: &str = "20180929-KIFUWARABECUP-0";

/// 対局変数。
pub struct Game {
    /// 汎用的に利用できるハッシュマップ。
    #[allow(dead_code)]
    pub properties: HashMap<String, String>,
    pub game_summary: GameSummary,
}
impl Game {
    pub fn new() -> Game {
        Game {
            properties: HashMap::new(),
            game_summary: GameSummary::new(),
        }
    }
}

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

    /// 対局間で共有する。 <対局番号,変数>
    pub static ref GAME_MAP: RwLock<HashMap<i64, Game>> = RwLock::new(HashMap::new());
}

pub fn set_player_name(player_num: i64, player_name: &str) {
    CLIENT_MAP
        .try_write()
        .unwrap()
        .get_mut(&player_num)
        .unwrap()
        .properties
        .insert("playerName".to_string(), player_name.to_string());
}

pub fn get_player_name(player_num: i64) -> String {
    CLIENT_MAP
        .try_read()
        .expect("CLIENT_MAP.try_write()")
        .get(&player_num)
        .unwrap()
        .properties
        .get("playerName")
        .unwrap_or(&"playerName not found.".to_string())
        .to_string()
}

pub fn set_player_to_game_room(player_num: i64, game_number: i64) {
    CLIENT_MAP
        .try_write()
        .expect("CLIENT_MAP.try_write()")
        .get_mut(&player_num)
        .unwrap()
        .properties
        .insert("gameRoom".to_string(), game_number.to_string());
}

pub fn set_player_state(player_num: i64, state: &str) {
    CLIENT_MAP
        .try_write()
        .expect("CLIENT_MAP.try_write()")
        .get_mut(&player_num)
        .unwrap()
        .properties
        .insert("state".to_string(), state.to_string());
}

/**
 * 2人待っていれば、マッチングしようぜ☆（＾ｑ＾）
 */
pub fn setup_2player_to_match() {
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

        println!(
            "プレイヤー0: {} を ゲームルーム {} へ移動。",
            player_num0, game_number
        );
        set_player_to_game_room(player_num0, game_number as i64);
        set_player_state(player_num0, "starting");

        println!(
            "プレイヤー1: {} を ゲームルーム {} へ移動。",
            player_num1, game_number
        );
        set_player_to_game_room(player_num1, game_number as i64);
        set_player_state(player_num1, "starting");

        println!("マッチング終わり。");
    }
}

pub fn is_state(player_num: i64, state: &str) -> bool {
    state == CLIENT_MAP
        .try_read()
        .expect("CLIENT_MAP.try_read()")
        .get(&player_num)
        .unwrap()
        .properties
        .get(&state.to_string())
        .unwrap()
}

/**
 * 対局室を１つ作成。
 *
 * # Returns.
 * ゲーム部屋番号。
 */
pub fn create_game(player_num0: i64, player_num1: i64) -> usize {
    let game_number;

    println!("プレイヤーデータ0: {} 取得", player_num0);
    let player_name0 = get_player_name(player_num0);

    println!("プレイヤーデータ1: {} 取得", player_num1);
    let player_name1 = get_player_name(player_num1);

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

/**
 * メッセージ作成。
 */
pub fn get_game_summary_string(game_num: i64) -> String {
    GAME_MAP.try_read().unwrap()[&game_num]
        .game_summary
        .to_string_ln()
}
