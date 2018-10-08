use models::player::Player;
use server_controller_impl::PLAYER_MAP;

pub struct PlayerMapUtil {}
impl PlayerMapUtil {
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
    pub fn set_state(connection_number: i64, state: &str) {
        PLAYER_MAP
            .try_write()
            .unwrap()
            .get_mut(&connection_number)
            .expect("sending-state")
            .set_state(&state.to_string());
    }
    pub fn get_entry_game(connection_number: i64) -> i64 {
        PLAYER_MAP
            .try_read()
            .unwrap()
            .get(&connection_number)
            .expect("player-get-entry-game")
            .get_entry_game()
    }
}
