//use kifuwarabe_server::*;
use client_handle_impl::*;

pub struct PlayerUtil {}
impl PlayerUtil {
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
    pub fn set_state(player_num: i64, status: &str) {
        PLAYER_MAP
            .try_write()
            .unwrap()
            .get_mut(&player_num)
            .expect("player-set-state")
            .set_state(&status.to_string());
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