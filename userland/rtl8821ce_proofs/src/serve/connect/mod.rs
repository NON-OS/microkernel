// NONOS Operating System (AGPL-3.0-or-later)

//! The connect stage, mounted file by file as src/serve/connect/mod.rs lays
//! it out, so its proofs can sit inside it.

#![allow(dead_code)] // what the proofs do not call of the mounted files

#[path = "../../../../capsule_driver_rtl8821ce/src/serve/connect/attempt.rs"]
mod attempt;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/connect/consts.rs"]
mod consts;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/connect/ends.rs"]
mod ends;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/connect/entropy.rs"]
mod entropy;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/connect/hunt/mod.rs"]
mod hunt;
#[cfg(test)]
mod hunt_tests;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/connect/install.rs"]
pub mod install;
#[cfg(test)]
mod install_rig;
#[cfg(test)]
mod install_tests;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/connect/join.rs"]
mod join;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/connect/leave.rs"]
mod leave;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/connect/media.rs"]
mod media;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/connect/pace.rs"]
mod pace;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/connect/progress.rs"]
mod progress;
#[cfg(test)]
mod rank_tests;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/connect/ready.rs"]
mod ready;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/connect/run.rs"]
mod run;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/connect/session.rs"]
pub mod session;
// Mounted once, in mounts.rs; `super::X` reaches them here.
pub use crate::connect_probe as probe;
pub use crate::connect_request as request;
pub use crate::connect_result as result;

pub(super) use entropy::reseed;
pub(super) use join::connect;
pub(super) use leave::disconnect;
pub(super) use media::set_media_no_link;
pub(super) use result::ConnectResult;
pub(super) use session::Session;
