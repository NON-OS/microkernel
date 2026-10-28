// NONOS Operating System (AGPL-3.0-or-later)

//! The receive path. The descriptor (rtw88 rx.h, the same 24 bytes on every
//! 11ac chip), the ring, its registers and the poll are the RTL8821CE's; the
//! PHY status the signal is read from is the 8822C's own (`signal`).

#[path = "../../../capsule_driver_rtl8821ce/src/rx/desc/mod.rs"]
pub mod desc;
#[path = "../../../capsule_driver_rtl8821ce/src/rx/fcs.rs"]
pub mod fcs;
#[path = "../../../capsule_driver_rtl8821ce/src/rx/poll/mod.rs"]
mod poll;
#[path = "../../../capsule_driver_rtl8821ce/src/rx/regs.rs"]
pub mod regs;
#[path = "../../../capsule_driver_rtl8821ce/src/rx/ring.rs"]
pub mod ring;
#[path = "../../../capsule_driver_rtl8821ce/src/rx/setup.rs"]
mod setup;
pub mod signal;

#[cfg(test)]
pub use poll::poll_one;
pub use poll::poll_one_info;
pub use setup::program;
