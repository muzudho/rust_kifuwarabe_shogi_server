pub enum Turn {
    /** 先手 */
    Black,
    /** 後手 */
    White
}

/**
 * CSAプロトコルの Game_Summary
 */
pub struct GameSummary {
    game_id: String,
    name_arr: [String;2],
    turn: Turn,
}
impl GameSummary {
    pub fn new() -> GameSummary {
        GameSummary {
            game_id: "".to_string(),
            name_arr: ["".to_string(),"".to_string()],
            turn: Turn::Black,
        }
    }

    pub fn get_game_id(&self) -> &str{
        &self.game_id
    }
    pub fn set_game_id(&mut self, value:&str) -> &mut GameSummary{
        self.game_id = value.to_string();
        self
    }

    pub fn get_name(&self, turn:Turn) -> &str{
        &self.name_arr[turn as usize]
    }
    pub fn set_name_arr(&mut self, value:[String;2]) -> &mut GameSummary{
        self.name_arr = value;
        self
    }

    pub fn get_turn(&self) -> &str{
        &self.game_id
    }
    pub fn set_turn(&mut self, value:Turn) -> &mut GameSummary{
        self.turn = value;
        self
    }

    pub fn to_string_ln(&self) -> String {
        format!(r#"BEGIN Game_Summary
Protocol_Version:1.2
Protocol_Mode:Server
Format:Shogi 1.0
Declaration:Jishogi 1.1
Game_ID:{}
Name+:{}
Name-:{}
Your_Turn:{}
Rematch_On_Draw:NO
To_Move:+
Max_Moves:256
BEGIN Time
Time_Unit:1sec
Total_Time:600
Byoyomi:10
Least_Time_Per_Move:1
END Time
BEGIN Position
P1-KY-KE-GI-KI-OU-KI-GI-KE-KY
P2 * -HI *  *  *  *  * -KA * 
P3-FU-FU-FU-FU-FU-FU-FU-FU-FU
P4 *  *  *  *  *  *  *  *  * 
P5 *  *  *  *  *  *  *  *  * 
P6 *  *  *  *  *  *  *  *  * 
P7+FU+FU+FU+FU+FU+FU+FU+FU+FU
P8 * +KA *  *  *  *  * +HI * 
P9+KY+KE+GI+KI+OU+KI+GI+KE+KY
P+
P-
+
END Position
END Game_Summary
"#  // 最後に改行が必要。
        , self.game_id
        , self.get_name(Turn::Black)
        , self.get_name(Turn::White)
        , match self.turn {
            Turn::Black => "+",
            Turn::White => "-",
        }
        )
    }
}
