// NONOS Operating System (AGPL-3.0-or-later)

//! The serving stage: net_core's link operations and the Wi-Fi control
//! family (scan, join, status), the background scanner and the join. All of
//! it is the RTL8821CE's, which drives the radio only through the link, the
//! shared core and `phy::channel::set_rf`; the radio's bring-up (`radio`)
//! is the 8822C's.

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
