use models::game::*;
use server_controller_impl::GAME_MAP;
// use server_controller_impl::LOBBY;
// use server_controller_impl::*;
use utils::game_utils::*;
use utils::lobby_utils::*;
use utils::player_utils::*;

/// 試し。
const GAME_NAME_TAMESI: &str = "20180929-KIFUWARABECUP-0";

pub struct SomeUtil {}
impl SomeUtil {
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
            let game_number = SomeUtil::create_game(player_num0, player_num1);
            println!(
                "対局室 {} を作成。 {} vs {}.",
                game_number, player_num0, player_num1
            );

            println!(
                "プレイヤー0: {} を ゲームルーム {} へ移動。",
                player_num0, game_number
            );

            // ゲーム部屋番号を設定。
            PlayerUtil::set_game(player_num0, game_number as i64);
            // 接続者のステータスを設定。
            PlayerUtil::set_state(player_num0, &"starting".to_string());

            println!(
                "プレイヤー1: {} を ゲームルーム {} へ移動。",
                player_num1, game_number
            );
            // ゲーム部屋番号を設定。
            PlayerUtil::set_game(player_num1, game_number as i64);
            // 接続者のステータスを設定。
            PlayerUtil::set_state(player_num1, &"starting".to_string());

            println!("マッチング終わり。");
        }
    }

    /// 対局室を１つ作成。
    /// 
    /// # Returns.
    /// 
    /// ゲーム部屋番号。
    pub fn create_game(player_num0: i64, player_num1: i64) -> usize {
        println!("プレイヤーデータ0: {} 取得", player_num0);
        let player_name0 = PlayerUtil::get_name(player_num0);

        println!("プレイヤーデータ1: {} 取得", player_num1);
        let player_name1 = PlayerUtil::get_name(player_num1);

        println!("対局室オブジェクト生成");
        let mut game = Game::new();

        // とりあえず
        println!("ゲームIDセット");
        game.set_name(GAME_NAME_TAMESI.to_string());

        println!("名前配列セット");
        GameUtil::set_player0_name(&mut game, &player_name0);
        GameUtil::set_player1_name(&mut game, &player_name1);

        println!("ターン セット");
        game.set_turn(Turn::Black);

        let game_number = GAME_MAP.try_write().unwrap().len();
        {
            println!("部屋番号 {}", game_number);

            println!("部屋挿入");
            GameUtil::set_number(&mut game, game_number as i64);
            GameUtil::insert(game_number as i64, game);
        }

        game_number
    }
}
