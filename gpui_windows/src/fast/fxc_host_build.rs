// Modified by Longbridge for gpui-fast.
//! Release builds for a Windows target compile HLSL with FXC on any host, so
//! a Windows client can be cross-compiled.
//!
//! On a non-Windows host, `GPUI_FXC_PATH` names `fxc.exe` and FXC runs under
//! Wine. FXC reads an argument with a leading `/` as an option, so paths are
//! passed through Wine's `Z:` drive, which maps the host's root.

use std::process::Command;

/// Whether this build embeds FXC bytecode: a release build for Windows.
/// `CARGO_CFG_*` describe the target, so this holds when cross-compiling too.
pub fn compiles_shaders() -> bool {
    std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows")
        && std::env::var_os("CARGO_CFG_DEBUG_ASSERTIONS").is_none()
}

/// `fxc`, run as this host runs a Windows program.
pub fn on_host(fxc: Command) -> Command {
    if cfg!(windows) {
        return fxc;
    }
    let mut wine = Command::new("wine");
    wine.arg(fxc.get_program());
    wine
}

/// `path` as the FXC this host runs reads it.
pub fn path(path: &str) -> String {
    if cfg!(windows) {
        path.to_owned()
    } else {
        format!("Z:{}", path.replace('/', "\\"))
    }
}

#[cfg(windows)]
pub use windows_registry as registry;

/// Off Windows there is no registry, so the Windows SDK lookup fails as it does
/// without an SDK, and `GPUI_FXC_PATH` must name FXC.
#[cfg(not(windows))]
pub mod registry {
    pub struct Key;

    pub static LOCAL_MACHINE: Key = Key;

    const NO_REGISTRY: &str = "this host has no Windows registry";

    impl Key {
        pub fn open(&self, _path: &str) -> Result<Key, &'static str> {
            Err(NO_REGISTRY)
        }

        pub fn get_string(&self, _name: &str) -> Result<String, &'static str> {
            Err(NO_REGISTRY)
        }
    }
}
