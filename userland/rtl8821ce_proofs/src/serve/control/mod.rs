// NONOS Operating System (AGPL-3.0-or-later)

//! The control requests, mounted file by file as src/serve/control/mod.rs
//! lays them out; the status body is public for the status proofs.

#[path = "../../../../capsule_driver_rtl8821ce/src/serve/control/dispatch.rs"]
mod dispatch;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/control/join.rs"]
mod join;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/control/leave.rs"]
mod leave;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/control/link_op.rs"]
mod link_op;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/control/ops.rs"]
mod ops;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/control/scan.rs"]
mod scan;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/control/status_body.rs"]
pub mod status_body;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/control/status_op.rs"]
mod status_op;

pub(super) use dispatch::control;
use ops::WIFI_HDR;
