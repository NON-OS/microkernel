// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the RTL8821CE driver: its real source, included by path and
//! driven against a modeled register file, checked without hardware.

extern crate alloc;

#[path = "../../capsule_driver_rtl8821ce/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_rtl8821ce/src/coex/mod.rs"]
pub mod coex;
#[cfg(test)]
mod coex_tests;
#[path = "../../capsule_driver_rtl8821ce/src/pwr/mod.rs"]
pub mod pwr;
#[path = "../../capsule_driver_rtl8821ce/src/regs.rs"]
pub mod regs;

#[cfg(test)]
mod ddma_tests;
#[cfg(test)]
mod download_tests;
#[path = "../../capsule_driver_rtl8821ce/src/efuse.rs"]
pub mod efuse;
#[cfg(test)]
mod efuse_tests;
#[path = "../../capsule_driver_rtl8821ce/src/fw/mod.rs"]
pub mod fw;
#[cfg(test)]
mod fw_tests;
#[path = "../../capsule_driver_rtl8821ce/src/h2c/mod.rs"]
pub mod h2c;
#[cfg(test)]
mod h2c_tests;
#[path = "../../capsule_driver_rtl8821ce/src/link.rs"]
pub mod link;
#[cfg(test)]
mod link_tests;
#[cfg(test)]
mod linkport_tests;
#[path = "../../capsule_driver_rtl8821ce/src/assoc.rs"]
pub mod assoc;
#[cfg(test)]
mod assoc_tests;
// The connect request and result codes of the serve stage; the rest of the
// stage needs the kernel.
#[path = "../../capsule_driver_rtl8821ce/src/serve/connect/request.rs"]
pub mod connect_request;
#[path = "../../capsule_driver_rtl8821ce/src/serve/connect/result.rs"]
pub mod connect_result;
#[path = "../../capsule_driver_rtl8821ce/src/serve/connect/probe.rs"]
pub mod connect_probe;
#[cfg(test)]
mod connect_tests;
// The answer the serve loop sends a frame neither request family takes.
#[path = "../../capsule_driver_rtl8821ce/src/serve/refuse.rs"]
pub mod serve_refuse;
#[cfg(test)]
mod serve_refuse_tests;
// The access point's half of the key handshakes, shared with the core proofs;
// the link proofs use only part of it.
#[cfg(test)]
#[allow(dead_code)]
#[path = "../../nonos_wifi_core_proofs/src/ap_sim.rs"]
mod ap_sim;
#[path = "../../capsule_driver_rtl8821ce/src/mac/mod.rs"]
pub mod mac;
#[cfg(test)]
mod mac_tests;
#[cfg(test)]
mod mac_trx_tests;
#[path = "../../capsule_driver_rtl8821ce/src/phy/mod.rs"]
pub mod phy;
#[cfg(test)]
mod phy_tests;
#[cfg(test)]
mod phy_rxpath_tests;
#[cfg(test)]
mod prep_tests;
#[cfg(test)]
mod pwr_cycle_tests;
#[cfg(test)]
mod pwr_mock;
#[cfg(test)]
mod pwr_switch_tests;
#[cfg(test)]
mod pwr_tests;
#[path = "../../capsule_driver_rtl8821ce/src/ring/mod.rs"]
pub mod ring;
#[path = "../../capsule_driver_rtl8821ce/src/rx/mod.rs"]
pub mod rx;
#[cfg(test)]
mod rx_tests;
#[path = "../../capsule_driver_rtl8821ce/src/scan.rs"]
pub mod scan;
#[cfg(test)]
mod scan_tests;
#[path = "../../capsule_driver_rtl8821ce/src/sec.rs"]
pub mod sec;
#[cfg(test)]
mod sec_tests;
#[cfg(test)]
mod sections_tests;
#[cfg(test)]
mod staging_tests;
#[cfg(test)]
mod tables_tests;
#[path = "../../capsule_driver_rtl8821ce/src/tx/mod.rs"]
pub mod tx;
#[cfg(test)]
mod tx_tests;
