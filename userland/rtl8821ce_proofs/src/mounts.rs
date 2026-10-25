// NONOS Operating System (AGPL-3.0-or-later)
//! The driver sources the proofs run, each included by path.

#[path = "../../capsule_driver_rtl8821ce/src/assoc/mod.rs"]
pub mod assoc;
#[path = "fake_clock.rs"]
pub mod clock;
#[path = "../../capsule_driver_rtl8821ce/src/coex/mod.rs"]
pub mod coex;
#[path = "../../capsule_driver_rtl8821ce/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_rtl8821ce/src/efuse/mod.rs"]
pub mod efuse;
#[path = "../../capsule_driver_rtl8821ce/src/fw/mod.rs"]
pub mod fw;
#[path = "../../capsule_driver_rtl8821ce/src/h2c/mod.rs"]
pub mod h2c;
#[path = "../../capsule_driver_rtl8821ce/src/link.rs"]
pub mod link;
#[path = "../../capsule_driver_rtl8821ce/src/pwr/mod.rs"]
pub mod pwr;
#[path = "../../capsule_driver_rtl8821ce/src/regs/mod.rs"]
pub mod regs;
// The connect request, probe and result codes, mounted once here; the serve
// stage's mirror (serve/connect/mod.rs) reaches them under its own names.
#[path = "../../capsule_driver_rtl8821ce/src/serve/connect/probe.rs"]
pub mod connect_probe;
#[path = "../../capsule_driver_rtl8821ce/src/serve/connect/request.rs"]
pub mod connect_request;
#[path = "../../capsule_driver_rtl8821ce/src/serve/connect/result/mod.rs"]
pub mod connect_result;
#[path = "../../capsule_driver_rtl8821ce/src/mac/mod.rs"]
pub mod mac;
#[path = "../../capsule_driver_rtl8821ce/src/pcie/mod.rs"]
pub mod pcie;
#[path = "../../capsule_driver_rtl8821ce/src/phy/mod.rs"]
pub mod phy;
#[path = "../../capsule_driver_rtl8821ce/src/ring/mod.rs"]
pub mod ring;
#[path = "../../capsule_driver_rtl8821ce/src/rx/mod.rs"]
pub mod rx;
#[path = "../../capsule_driver_rtl8821ce/src/scan.rs"]
pub mod scan;
#[path = "../../capsule_driver_rtl8821ce/src/sec.rs"]
pub mod sec;
#[path = "../../capsule_driver_rtl8821ce/src/status.rs"]
pub mod status;
#[path = "../../capsule_driver_rtl8821ce/src/tx/mod.rs"]
pub mod tx;
#[path = "../../capsule_driver_rtl8821ce/src/wait.rs"]
pub mod wait;
// The bring-up files the serve stage reaches; their broker calls are refused
// by the shim, so the proofs build the radio over host memory instead.
#[path = "../../capsule_driver_rtl8821ce/src/discover.rs"]
pub mod discover;
#[path = "../../capsule_driver_rtl8821ce/src/fwload/mod.rs"]
pub mod fwload;
#[path = "../../capsule_driver_rtl8821ce/src/setup/mod.rs"]
pub mod setup;
#[path = "../../capsule_driver_rtl8821ce/src/station.rs"]
pub mod station;
// Two of the serve stage's files, under the names their proofs use.
pub use crate::serve::control::status_body as serve_status;
pub use crate::serve::refuse as serve_refuse;
