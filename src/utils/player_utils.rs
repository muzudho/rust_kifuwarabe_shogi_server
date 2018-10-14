//use kifuwarabe_server::*;
use client_handle_impl::*;
use models::player::Player;
use client_handle_impl::PLAYER_MAP;

pub struct PlayerUtil {}
impl PlayerUtil {
    // ハッシュマップに追加。
    pub fn insert(connection_number: i64, player: Player) {
        PLAYER_MAP
            .try_write()
            .unwrap()
            .insert(connection_number, player);
    }
    // プレイヤー名をセット。
    pub fn set_name(connection_number: i64, player_name: &str) {
        PLAYER_MAP
            .try_write()
            .unwrap()
            .get_mut(&connection_number)
            .unwrap()
            .set_name(player_name.to_string());
    }
    // パスワードをセット。
    pub fn set_password(connection_number: i64, password: &str) {
        PLAYER_MAP
            .try_write()
            .unwrap()
            .get_mut(&connection_number)
            .unwrap()
            .set_password(password);
    }
    pub fn get_entry_game(connection_number: i64) -> i64 {
        PLAYER_MAP
            .try_read()
            .unwrap()
            .get(&connection_number)
            .expect("player-get-entry-game")
            .get_entry_game()
    }

    pub fn get_name(player_num: i64) -> String {
        PLAYER_MAP
            .try_read()
            .unwrap()
            .get(&player_num)
            .unwrap()
            .get_name()
            .to_string()
    }
    pub fn set_game(player_num: i64, game_num: i64) {
        PLAYER_MAP
            .try_write()
            .unwrap()
            .get_mut(&player_num)
            .expect("player-set-game")
            .set_entry_game(game_num)
    }
    pub fn set_state(connection_number: i64, state: &str) {
        PLAYER_MAP
            .try_write()
            .unwrap()
            .get_mut(&connection_number)
            .expect("player-set-state")
            .set_state(&state.to_string());
    }
    pub fn is_state(player_num: i64, state: &str) -> bool {
        //println!("is_state: {}. expected: {}.", player_num, state);

        // 接続者のステータス。
        let player_state;
        {
            player_state = PLAYER_MAP
                .try_read()
                .unwrap()
                .get(&player_num)
                .expect("is-state")
                .get_state();
        }

        // 比較。
        player_state == state

        /*
        match CLIENT_MAP
            .try_read()
            .expect("CLIENT_MAP.try_read()")
            .get(&player_num)
            .unwrap()
            .properties
            .get(&state.to_string()) {
                Some(n) => {state == n},
                None => {false},
            }
    */
        /*
        state == CLIENT_MAP
            .try_read()
            .expect("CLIENT_MAP.try_read()")
            .get(&player_num)
            .unwrap()
            .properties[&state.to_string()]
        */
    }
}
