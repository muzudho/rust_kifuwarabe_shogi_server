use kifuwarabe_shell::diagram::*;
use client_handle_impl::SHELL_MAP;
use kifuwarabe_shell::shell::*;
use models::shell_var::*;

pub struct ShellMapUtils {}
impl ShellMapUtils {
    pub fn insert(connection_number: i64, shell: Shell<ShellVar>) {
        SHELL_MAP
            .try_write()
            .unwrap()
            .insert(connection_number, shell);
    }

    pub fn get_current(connection_number: i64) -> String {
        SHELL_MAP
            .try_read()
            .expect("shell_map_utils.rs/ShellMapUtils/get_current/SHELL_MAP.try_write()")
            .get(&connection_number)
            .unwrap_or_else(|| panic!("shell_map_utils.rs/ShellMapUtils/get_current/get({})", &connection_number))
            .get_current()
    }

    pub fn forward(connection_number: i64, diagram: &Diagram<ShellVar>, next: &str) {
        SHELL_MAP
            .try_write()
            .expect("shell_map_utils.rs/ShellMapUtils/forward/SHELL_MAP.try_write()")
            .get_mut(&connection_number)
            //.expect(&format!("shell_map_utils.rs/ShellMapUtils/forward/get_mut({})", &connection_number))
            .unwrap_or_else(|| panic!("shell_map_utils.rs/ShellMapUtils/forward/get_mut({})", &connection_number))
            .forward_force(diagram, next);
    }
}
