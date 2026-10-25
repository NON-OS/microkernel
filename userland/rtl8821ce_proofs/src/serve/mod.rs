// NONOS Operating System (AGPL-3.0-or-later)

//! The driver's serve stage, mounted file by file as src/serve/mod.rs lays it
//! out, so the proofs can sit inside it and reach its `pub(super)` items.

#![allow(dead_code)] // what the proofs do not call of the mounted files

#[path = "../../../capsule_driver_rtl8821ce/src/serve/after.rs"]
mod after;
#[path = "../../../capsule_driver_rtl8821ce/src/serve/answer.rs"]
mod answer;
pub mod connect;
pub mod control;
#[path = "../../../capsule_driver_rtl8821ce/src/serve/gone.rs"]
mod gone;
#[path = "../../../capsule_driver_rtl8821ce/src/serve/loss.rs"]
mod loss;
#[path = "../../../capsule_driver_rtl8821ce/src/serve/net_core.rs"]
mod net_core;
#[path = "../../../capsule_driver_rtl8821ce/src/serve/radio/mod.rs"]
mod radio;
#[path = "../../../capsule_driver_rtl8821ce/src/serve/refuse.rs"]
pub mod refuse;
#[path = "../../../capsule_driver_rtl8821ce/src/serve/run.rs"]
mod run;
mod scanner;
#[path = "../../../capsule_driver_rtl8821ce/src/serve/stage.rs"]
mod stage;
#[path = "../../../capsule_driver_rtl8821ce/src/serve/tuning.rs"]
mod tuning;

#[cfg(test)]
mod gone_tests;
#[cfg(test)]
mod host_radio;
#[cfg(test)]
mod hostile_tests;

pub use run::run;
pub use stage::Stage;
use tuning::{DEFAULT_CHANNEL, SCAN_CHANNELS, SCAN_FRAME_MAX};
