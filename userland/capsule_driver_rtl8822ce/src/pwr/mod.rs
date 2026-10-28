// NONOS Operating System (AGPL-3.0-or-later)

//! The MAC power sequences. The engine (command shape, executor, poll, the
//! pre-system and system configuration, the warm-card cycle and Bus Master
//! only after it) is the same on every rtw88 11ac PCIe chip and is the
//! RTL8821CE's; the 8822C's own tables are `cycle` and `tables`.

#[path = "../../../capsule_driver_rtl8821ce/src/pwr/command.rs"]
pub mod command;
mod cycle;
#[path = "../../../capsule_driver_rtl8821ce/src/pwr/master.rs"]
mod master;
#[path = "../../../capsule_driver_rtl8821ce/src/pwr/poll.rs"]
mod poll;
#[path = "../../../capsule_driver_rtl8821ce/src/pwr/presys.rs"]
mod presys;
#[path = "../../../capsule_driver_rtl8821ce/src/pwr/rpwm.rs"]
mod rpwm;
#[path = "../../../capsule_driver_rtl8821ce/src/pwr/run.rs"]
mod run;
#[path = "../../../capsule_driver_rtl8821ce/src/pwr/switch.rs"]
mod switch;
#[path = "../../../capsule_driver_rtl8821ce/src/pwr/sysinit.rs"]
mod sysinit;
mod tables;

pub use cycle::{CARD_DISABLE, CARD_EMULATE, CR_UNPOWERED};
pub use master::{power_on_then_master, NoMaster};
pub use run::run_pwr_seq;
pub use switch::{power_on, PowerOn};
pub use tables::CARD_ENABLE;
