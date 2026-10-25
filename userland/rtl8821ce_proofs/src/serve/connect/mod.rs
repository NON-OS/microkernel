// NONOS Operating System (AGPL-3.0-or-later)

//! The connect stage's answer to a finished join, with the siblings it
//! reaches through `super`, mounted from the driver.

#![allow(dead_code)] // what the proofs do not call of the mounted files

#[path = "../../../../capsule_driver_rtl8821ce/src/serve/connect/consts.rs"]
mod consts;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/connect/ends.rs"]
mod ends;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/connect/install.rs"]
pub mod install;
#[cfg(test)]
mod install_rig;
#[cfg(test)]
mod install_tests;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/connect/media.rs"]
mod media;
// Mounted once, in mounts.rs; `super::result` reaches it here.
pub use crate::connect_result as result;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/connect/session.rs"]
pub mod session;
