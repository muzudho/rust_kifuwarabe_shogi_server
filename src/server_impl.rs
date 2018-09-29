use kifuwarabe_server::*;

pub fn set_player_name(player_num: i64, player_name: &str) {
    CLIENT_MAP
        .try_write()
        .unwrap()
        .get_mut(&player_num)
        .unwrap()
        .properties
        .insert("playerName".to_string(), player_name.to_string());
}

pub fn get_player_data(player_num: i64) -> String {
    CLIENT_MAP
        .try_read().expect("CLIENT_MAP.try_write()")
        .get(&player_num)
        .unwrap()
        .properties
        .get("playerName")
        .unwrap_or(&"playerName not found.".to_string())
        .to_string()
}

pub fn set_player_to_game_room(player_num: i64, game_number: i64){
    CLIENT_MAP
        .try_write().expect("CLIENT_MAP.try_write()")
        .get_mut(&player_num)
        .unwrap()
        .properties
        .insert("gameRoom".to_string(), game_number.to_string());
}

pub fn set_player_state(player_num: i64, state: &str){
    CLIENT_MAP
        .try_write().expect("CLIENT_MAP.try_write()")
        .get_mut(&player_num)
        .unwrap()
        .properties
        .insert("state".to_string(), state.to_string());
}
