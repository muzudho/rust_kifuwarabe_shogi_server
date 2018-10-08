//use kifuwarabe_server::*;
//use models::game::*;
use server_controller_impl::GAME_MAP;
//use server_controller_impl::LOBBY;
//use server_controller_impl::*;

/**
 * メッセージ作成。
 */
pub fn get_game_summary_string(game_num: i64) -> String {
    GAME_MAP.try_read().unwrap()[&game_num]
        .to_game_summary_string_ln()
}
