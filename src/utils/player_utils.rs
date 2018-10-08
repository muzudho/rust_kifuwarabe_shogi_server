//use kifuwarabe_server::*;
use server_controller_impl::*;

pub fn is_state(player_num: i64, state: &str) -> bool {
    //println!("is_state: {}. expected: {}.", player_num, state);

    // 接続者のステータス。
    let player_state;
    {
        player_state = PLAYER_MAP.try_read().unwrap().get(&player_num).expect("is-state").get_state();
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
