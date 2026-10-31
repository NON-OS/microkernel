// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's modules, each from where the capsule takes it; the clock is
//! the RTL8821CE proofs' (each reading moves it on); no entry point.

#[path = "../../capsule_driver_rtl8821ce/src/assoc/mod.rs"]
pub mod assoc;
#[path = "../../capsule_driver_rtl8821ce/src/bringup.rs"]
pub mod bringup;
#[path = "../../rtl8821ce_proofs/src/fake_clock.rs"]
pub mod clock;
#[path = "../../capsule_driver_rtl8822ce/src/coex/mod.rs"]
pub mod coex;
#[path = "../../capsule_driver_rtl8822ce/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_rtl8822ce/src/discover.rs"]
pub mod discover;
#[path = "../../capsule_driver_rtl8822ce/src/efuse/mod.rs"]
pub mod efuse;
#[path = "../../capsule_driver_rtl8822ce/src/fw/mod.rs"]
pub mod fw;
#[path = "../../capsule_driver_rtl8822ce/src/fwload/mod.rs"]
pub mod fwload;
#[path = "../../capsule_driver_rtl8821ce/src/h2c/mod.rs"]
pub mod h2c;
#[path = "../../capsule_driver_rtl8821ce/src/link.rs"]
pub mod link;
#[path = "../../capsule_driver_rtl8822ce/src/mac/mod.rs"]
pub mod mac;
#[path = "../../capsule_driver_rtl8822ce/src/pcie/mod.rs"]
pub mod pcie;
// The driver file is sealed for 0.9.3; these lints (phy/tables/cal_init.rs, phy/txpower/write.rs) are fixed at the next seal.
#[allow(clippy::mixed_case_hex_literals, clippy::type_complexity)]
#[path = "../../capsule_driver_rtl8822ce/src/phy/mod.rs"]
pub mod phy;
#[path = "../../capsule_driver_rtl8821ce/src/ps/mod.rs"]
pub mod ps;
#[path = "../../capsule_driver_rtl8822ce/src/pwr/mod.rs"]
pub mod pwr;
#[path = "../../capsule_driver_rtl8821ce/src/regs/mod.rs"]
pub mod regs;
#[path = "../../capsule_driver_rtl8821ce/src/ring/mod.rs"]
pub mod ring;
#[path = "../../capsule_driver_rtl8822ce/src/rx/mod.rs"]
pub mod rx;
#[path = "../../capsule_driver_rtl8821ce/src/scan.rs"]
pub mod scan;
#[path = "../../capsule_driver_rtl8821ce/src/sec.rs"]
pub mod sec;
#[path = "../../capsule_driver_rtl8821ce/src/setup/mod.rs"]
pub mod setup;
#[path = "../../capsule_driver_rtl8822ce/src/start_board.rs"]
pub mod start_board;
#[path = "../../capsule_driver_rtl8821ce/src/start_ready.rs"]
pub mod start_ready;
#[path = "../../capsule_driver_rtl8822ce/src/start_say.rs"]
pub mod start_say;
#[path = "../../capsule_driver_rtl8821ce/src/station.rs"]
pub mod station;
#[path = "../../capsule_driver_rtl8822ce/src/status/mod.rs"]
pub mod status;
#[path = "../../capsule_driver_rtl8821ce/src/tx/mod.rs"]
pub mod tx;
#[path = "../../capsule_driver_rtl8821ce/src/wait.rs"]
pub mod wait;
