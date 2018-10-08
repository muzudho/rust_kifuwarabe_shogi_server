use kifuwarabe_shell::shell::*;
use server_controller_impl::SHELL_MAP;
use models::shell_var::*;

pub struct ShellMapUtils {}
impl ShellMapUtils {
    pub fn insert(connection_number: i64, shell: Shell<ShellVar>) {
        SHELL_MAP
            .try_write()
            .unwrap()
            .insert(connection_number, shell);
    }
}
