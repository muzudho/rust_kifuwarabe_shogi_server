/**
 * CSAプロトコルの Game_Summary
 */
pub struct GameSummary {
    game_id: String
}
impl GameSummary {
    pub fn new() -> GameSummary {
        GameSummary {
            game_id: "".to_string(),
        }
    }
    pub fn get_game_id(&self) -> &str{
        &self.game_id
    }
    pub fn set_game_id(&mut self, value:&str){
        self.game_id = value.to_string()
    }
    pub fn to_string_ln(&self) -> &str {
        r#"BEGIN Game_Summary
Protocol_Version:1.2
Protocol_Mode:Server
Format:Shogi 1.0
Declaration:Jishogi 1.1
Game_ID:{}
Name+:kifuwarabe
Name-:KITSUNE
Your_Turn:+
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
    }
}
