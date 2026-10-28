// NONOS Operating System (AGPL-3.0-or-later)

//! RTL8822CE wifi driver. The chip's own files are here: power sequences,
//! efuse, MAC init, PHY with its 2x2 RF paths and DAC calibration, receive
//! signal, coexistence and PCIe. Every module that is the same for any rtw88
//! 11ac PCIe chip is capsule_driver_rtl8821ce's, mounted by `#[path]`.

#![no_std]
#![no_main]

extern crate alloc;

#[path = "../../capsule_driver_rtl8821ce/src/assoc/mod.rs"]
mod assoc;
#[path = "../../capsule_driver_rtl8821ce/src/bringup.rs"]
mod bringup;
#[path = "../../capsule_driver_rtl8821ce/src/clock.rs"]
mod clock;
mod coex;
mod constants;
mod discover;
mod efuse;
mod fw;
mod fwload;
#[path = "../../capsule_driver_rtl8821ce/src/h2c/mod.rs"]
mod h2c;
#[path = "../../capsule_driver_rtl8821ce/src/link.rs"]
mod link;
mod mac;
mod pcie;
mod phy;
#[path = "../../capsule_driver_rtl8821ce/src/ps/mod.rs"]
mod ps;
mod pwr;
#[path = "../../capsule_driver_rtl8821ce/src/regs/mod.rs"]
mod regs;
#[path = "../../capsule_driver_rtl8821ce/src/ring/mod.rs"]
mod ring;
mod rx;
#[path = "../../capsule_driver_rtl8821ce/src/scan.rs"]
mod scan;
#[path = "../../capsule_driver_rtl8821ce/src/sec.rs"]
mod sec;
mod serve;
#[path = "../../capsule_driver_rtl8821ce/src/setup/mod.rs"]
mod setup;
mod start;
mod start_board;
#[path = "../../capsule_driver_rtl8821ce/src/start_ready.rs"]
mod start_ready;
mod start_say;
#[path = "../../capsule_driver_rtl8821ce/src/station.rs"]
mod station;
mod status;
#[path = "../../capsule_driver_rtl8821ce/src/tx/mod.rs"]
mod tx;
#[path = "../../capsule_driver_rtl8821ce/src/wait.rs"]
mod wait;

use nonos_libc::{heap_init, mk_exit};

/// # Safety
/// The capsule entry point. The runtime jumps here once, on a fresh stack, with
/// no prior Rust state; it must not be called from Rust. It initialises the heap
/// before any allocation and never returns.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    // Serve whatever stage bring-up reached, so the panel can read it.
    let (mapped, stage) = start::bring_up();
    serve::run(mapped, stage);
}
