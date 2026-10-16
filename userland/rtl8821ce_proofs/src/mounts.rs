// NONOS Operating System (AGPL-3.0-or-later)
//! The driver sources the proofs run, each included by path.

#[path = "../../capsule_driver_rtl8821ce/src/assoc/mod.rs"]
pub mod assoc;
#[path = "../../capsule_driver_rtl8821ce/src/coex/mod.rs"]
pub mod coex;
#[path = "../../capsule_driver_rtl8821ce/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_rtl8821ce/src/efuse.rs"]
pub mod efuse;
#[path = "../../capsule_driver_rtl8821ce/src/fw/mod.rs"]
pub mod fw;
#[path = "../../capsule_driver_rtl8821ce/src/h2c/mod.rs"]
pub mod h2c;
#[path = "../../capsule_driver_rtl8821ce/src/link.rs"]
pub mod link;
#[path = "../../capsule_driver_rtl8821ce/src/pwr/mod.rs"]
pub mod pwr;
#[path = "../../capsule_driver_rtl8821ce/src/regs.rs"]
pub mod regs;
// The connect request and result codes of the serve stage; the rest of the
// stage needs the kernel.
#[path = "../../capsule_driver_rtl8821ce/src/serve/connect/probe.rs"]
pub mod connect_probe;
#[path = "../../capsule_driver_rtl8821ce/src/serve/connect/request.rs"]
pub mod connect_request;
#[path = "../../capsule_driver_rtl8821ce/src/serve/connect/result.rs"]
pub mod connect_result;
#[path = "../../capsule_driver_rtl8821ce/src/mac/mod.rs"]
pub mod mac;
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
// The answer the serve loop sends a frame neither request family takes.
#[path = "../../capsule_driver_rtl8821ce/src/serve/refuse.rs"]
pub mod serve_refuse;
#[path = "../../capsule_driver_rtl8821ce/src/tx/mod.rs"]
pub mod tx;
