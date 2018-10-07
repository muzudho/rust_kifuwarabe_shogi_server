use kifuwarabe_shell::diagram::*;
//use kifuwarabe_shell::shell::*;
use server_controller_impl::*;
//use server_diagram_impl::*;
use models::shell_var::*;

// グローバル変数。
use std::sync::RwLock;
lazy_static! {
    // Shell と Diagram は名前被りがあるので、ファイルを分けている。
    // 一度作ってしまえば、あとは読込のみ。
    pub static ref DIAGRAM: RwLock<Diagram<ShellVar>> = RwLock::new(Diagram::new());
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

/// プレイヤー名を読み取った。
pub fn do_player_name(shell_var: &mut ShellVar, req: &Request, _res: &mut dyn Response) {
    let player_name = &req.get_groups()[0];

    // 接続者のログイン名を記録。
    {
        PLAYER_MAP.try_write().unwrap().get_mut(&shell_var.get_connection_number()).unwrap().set_name(player_name.to_string());
    }

    println!(
        "<{} do_player_name: {}",
        shell_var.get_connection_number(),
        player_name
    );
    // res.forward("#next");
}

/// 接続者前とパスワードを分解した。
/// 
/// - ログイン完了の通知をクライアントに返す。
/// - 対局はまだ付いていない。
pub fn do_password(shell_var: &mut ShellVar, req: &Request, _res: &mut dyn Response) {
    println!(
        "<{} do_password: {}",
        shell_var.get_connection_number(),
        req.get_groups()[0]
    );

    // 接続者のパスワードを記録。
    {
        PLAYER_MAP.try_write().unwrap().get_mut(&shell_var.get_connection_number()).unwrap().set_password(&req.get_groups()[0]);
    }

    // 応答メッセージ作成。
    let player_name;
    {
        player_name = PLAYER_MAP.try_read().unwrap().get(&shell_var.get_connection_number()).unwrap().get_name().to_string();
    }
    shell_var.set_message_to_client(&format!(
        r#"LOGIN:{} OK
"#, // 改行。
        player_name
    ));
    // res.forward("#next");
}
