// NONOS Operating System (AGPL-3.0-or-later)
//! The serve stage, mounted file by file as the capsule's serve/mod.rs lays
//! it out (the RTL8821CE's files, this driver's radio), so the PHY entry's
//! proofs can sit inside the radio and reach its `pub(super)` items.

#[path = "../../../capsule_driver_rtl8821ce/src/serve/after.rs"]
mod after;
#[path = "../../../capsule_driver_rtl8821ce/src/serve/answer.rs"]
mod answer;
#[path = "../../../capsule_driver_rtl8821ce/src/serve/connect/mod.rs"]
mod connect;
#[path = "../../../capsule_driver_rtl8821ce/src/serve/control/mod.rs"]
mod control;
#[path = "../../../capsule_driver_rtl8821ce/src/serve/gone.rs"]
mod gone;
#[path = "../../../capsule_driver_rtl8821ce/src/serve/loss.rs"]
mod loss;
#[path = "../../../capsule_driver_rtl8821ce/src/serve/net_core.rs"]
mod net_core;
mod radio;
#[path = "../../../capsule_driver_rtl8821ce/src/serve/refuse.rs"]
mod refuse;
#[path = "../../../capsule_driver_rtl8821ce/src/serve/run.rs"]
mod run;
#[path = "../../../capsule_driver_rtl8821ce/src/serve/scanner/mod.rs"]
mod scanner;
#[path = "../../../capsule_driver_rtl8821ce/src/serve/stage.rs"]
mod stage;
#[path = "../../../capsule_driver_rtl8821ce/src/serve/tuning.rs"]
mod tuning;

pub use run::run;
pub use stage::Stage;
use tuning::{DEFAULT_CHANNEL, SCAN_CHANNELS, SCAN_FRAME_MAX};
