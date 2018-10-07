use kifuwarabe_shell::diagram::*;
use kifuwarabe_shell::shell::*;

use server_diagram_impl::*;

const DIAGRAM_JSON_FILE: &str = "diagram.json";

/// 任意のオブジェクト。
pub struct ShellVar {
    connection_number: i64,
    /// どのフローが終わったか、識別する文字列。
    flow_message: String,
}
impl ShellVar {
    fn new() -> ShellVar {
        ShellVar {
            connection_number: -1,
            flow_message: "".to_string(),
        }
    }
    pub fn get_flow_message(&self) -> String {
        self.flow_message.to_string()
    }
    pub fn set_flow_message(&mut self, value:&str){
        self.flow_message = value.to_string();
    }
}

// グローバル変数。
use std::sync::RwLock;
lazy_static! {
    // 一度作ってしまえば、あとは読込のみ。
    static ref DIAGRAM: RwLock<Diagram<ShellVar>> = RwLock::new(Diagram::new());
}

/// グラフは１つ作れば、どのクライアントでも使いまわす。
pub fn setup_diagram() {
    let mut diagram = DIAGRAM.try_write().unwrap();

    // コントローラー登録。
    diagram.insert_fn("do_player_name", do_player_name);
    diagram.insert_fn("do_password", do_password);

    // ファイル読取。
    diagram.read_file(&DIAGRAM_JSON_FILE.to_string());
}

// クライアント１つごとに、１つのシェルを割り当てる。
pub fn execute_line_by_client(connection_number: i64, line: &str) -> String {
    // 任意のオブジェクト。
    let mut shell_var = ShellVar::new();
    shell_var.connection_number = connection_number;

    {
        // 実行。グラフと 任意のオブジェクトを渡す。
        let mut shell = Shell::new();
        let mut diagram = DIAGRAM.try_write().unwrap();
        shell.execute_line(&mut diagram, &mut shell_var, line);
    }

    shell_var.get_flow_message()
}

/**
 * プレイヤー名を読み取った。
 */
pub fn do_player_name(shell_var: &mut ShellVar, req: &Request, res: &mut dyn Response) {
    let player_name = &req.get_groups()[0];
    set_player_name(shell_var.connection_number, &player_name);
    println!("<{} do_player_name: {}", shell_var.connection_number, player_name);
    res.forward("next");
}

pub fn do_password(shell_var: &mut ShellVar, req: &Request, _res: &mut dyn Response) {
    println!("<{} do_password: {}", shell_var.connection_number, req.get_groups()[0]);
    shell_var.set_flow_message("loginEnd");
    // res.forward("next");
}
