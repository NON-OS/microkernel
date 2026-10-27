// NONOS Operating System (AGPL-3.0-or-later)

//! RTL8852BE Wi-Fi driver (rtw89, 802.11ax), a port of Linux v6.12's
//! `drivers/net/wireless/realtek/rtw89/` for this one chip. Start-up claims
//! the chip and brings it up in rtw89's order, reporting each stage; the
//! station above the frames is the shared `nonos_wifi_core`'s.

#![no_std]
#![no_main]

extern crate alloc;

mod bring;
mod chan;
mod clock;
mod coex;
mod discover;
mod dle;
mod dma;
mod efuse;
mod fw;
mod h2c;
mod mac;
mod pcie;
mod phy;
mod pwr;
mod regs;
mod rfk;
mod ring;
mod setup;
mod status;
mod wait;

use nonos_libc::{heap_init, mk_exit};

/// # Safety
/// The capsule entry point. The runtime jumps here once, on a fresh stack;
/// it initialises the heap before any allocation and never returns.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    mk_exit(0)
}
