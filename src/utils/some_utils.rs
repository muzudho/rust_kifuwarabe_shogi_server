use models::game::*;
use server_controller_impl::GAME_MAP;
use server_controller_impl::LOBBY;
use server_controller_impl::*;
use utils::lobby_utils::*;
use utils::player_utils::*;

/// 試し。
const GAME_NAME_TAMESI: &str = "20180929-KIFUWARABECUP-0";

/**
 * 2人待っていれば、マッチングしようぜ☆（＾ｑ＾）
 */
pub fn setup_2player_to_match() {
    if LobbyUtil::more_than_2player() {
        // 2人以上なら。
        let player_num0 = LobbyUtil::pop_player();
        let player_num1 = LobbyUtil::pop_player();
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

        // ゲーム部屋番号を設定。
        {
            PlayerUtil::set_game(player_num0, game_number as i64);
        }
        // 接続者のステータスを設定。
        {
            PLAYER_MAP
                .try_write()
                .unwrap()
                .get_mut(&player_num0)
                .expect("setup-state")
                .set_state(&"starting".to_string());
        }

        println!(
            "プレイヤー1: {} を ゲームルーム {} へ移動。",
            player_num1, game_number
        );
        // ゲーム部屋番号を設定。
        {
            PLAYER_MAP
                .try_write()
                .unwrap()
                .get_mut(&player_num1)
                .expect("setup-p1-entry-game")
                .set_entry_game(game_number as i64);
        }
        // 接続者のステータスを設定。
        {
            PLAYER_MAP
                .try_write()
                .unwrap()
                .get_mut(&player_num1)
                .expect("setup-state")
                .set_state(&"starting".to_string());
        }

        println!("マッチング終わり。");
    }
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
    let player_name0;
    {
        player_name0 = PLAYER_MAP
            .try_read()
            .unwrap()
            .get(&player_num0)
            .unwrap()
            .get_name()
            .to_string();
    }

    println!("プレイヤーデータ1: {} 取得", player_num1);
    let player_name1;
    {
        player_name1 = PLAYER_MAP
            .try_read()
            .unwrap()
            .get(&player_num1)
            .unwrap()
            .get_name()
            .to_string();
    }

    println!("対局室オブジェクト生成");
    let mut game = Game::new();

    {
        // とりあえず、
        println!("ゲームIDセット");
        game.set_name(GAME_NAME_TAMESI.to_string());

        println!("名前配列セット");
        game.get_mut_player0().set_name(player_name0.to_string());
        game.get_mut_player1().set_name(player_name1.to_string());

        println!("ターン セット");
        game.set_turn(Turn::Black);

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
