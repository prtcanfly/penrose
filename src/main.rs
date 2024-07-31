//! penrose :: minimal configuration
//!
//! This file will give you a functional if incredibly minimal window manager that
//! has multiple workspaces and simple client / workspace movement.
mod bindings;
mod layout;

use penrose::{
    core::{bindings::parse_keybindings_with_xmodmap, Config, WindowManager},
    extensions::hooks::add_ewmh_hooks,
    x11rb::RustConn,
    Result,
};
use tracing_subscriber::{self, prelude::*};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    let config = add_ewmh_hooks(Config {
        default_layouts: layout::layouts(),
        ..Config::default()
    });

    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(bindings::raw_key_bindings())?;
    let wm = WindowManager::new(config, key_bindings, bindings::mouse_bindings(), conn)?;

    wm.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindings_parse_correctly_with_xmodmap() {
        let res = parse_keybindings_with_xmodmap(bindings::raw_key_bindings());

        if let Err(e) = res {
            panic!("{e}");
        }
    }
}
