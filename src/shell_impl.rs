use kifuwarabe_shell::graph::*;
use kifuwarabe_shell::shell::*;

use server_impl::*;

const GRAPH_JSON_FILE: &str = "graph.json";

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
    static ref GRAPH: RwLock<Graph<ShellVar>> = RwLock::new(Graph::new());
}

/// グラフは１つ作れば、どのクライアントでも使いまわす。
pub fn setup_graph() {
    let mut graph = GRAPH.try_write().unwrap();

    // コントローラー登録。
    graph.insert_fn("do_player_name", do_player_name);
    graph.insert_fn("do_password", do_password);

    // ファイル読取。
    graph.read_graph_file(&GRAPH_JSON_FILE.to_string());
}

// クライアント１つごとに、１つのシェルを割り当てる。
pub fn execute_line_by_client(connection_number: i64, line: &str) -> String {
    // 任意のオブジェクト。
    let mut shell_var = ShellVar::new();
    shell_var.connection_number = connection_number;

    {
        // 実行。グラフと 任意のオブジェクトを渡す。
        let mut shell = Shell::new();
        let mut graph = GRAPH.try_write().unwrap();
        shell.execute_line(&mut graph, &mut shell_var, line);
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
