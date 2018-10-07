pub struct Player {
    /// 接続者番号。
    number: i64,
    /// 対局者名。
    name: String,
    /// ログイン時のパスワード。
    password: String,
}
impl Player {
    pub fn new() -> Player {
        Player {
            number: -1,
            name: "".to_string(),
            password: "".to_string(),
        }
    }

    pub fn get_number(&self) -> i64 {
        self.number
    }
    pub fn set_number(&mut self, value:i64) {
        self.number = value
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn set_name(&mut self, value:String) {
        self.name = value
    }

    pub fn get_password(&self) -> String {
        self.password.to_string()
    }
    pub fn set_password(&mut self, value: &str) {
        self.password = value.to_string();
    }

}
