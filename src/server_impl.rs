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
        .try_write().expect("CLIENT_MAP.try_write()")
        .get(&player_num)
        .unwrap()
        .properties
        .get("playerName")
        .unwrap_or(&"playerName not found.".to_string())
        .to_string()
}
