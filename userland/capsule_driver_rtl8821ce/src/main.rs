// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! RTL8821CE wifi driver. Start-up claims the chip, maps its registers, runs
//! the power-on sequence and reads the chip back, reporting on the boot
//! console how far it got. The firmware download (`fw`, `fwload`) and the
//! MAC, PHY, scan, association and security code (`mac`, `phy`, `scan`,
//! `assoc`, `sec`) run when the radio is brought up from `serve::radio`.

#![no_std]
#![no_main]

extern crate alloc;

mod assoc;
mod bringup;
mod clock;
mod coex;
mod constants;
mod discover;
mod efuse;
mod fw;
mod fwload;
mod link;
mod mac;
mod pcie;
mod phy;
mod pwr;
mod regs;
mod ring;
mod rx;
mod scan;
mod sec;
mod serve;
mod setup;
mod start;
mod start_board;
mod start_ready;
mod station;
mod status;
mod tx;
mod wait;

use nonos_libc::{heap_init, mk_exit};

use start::bring_up;

/// # Safety
/// The capsule entry point. The runtime jumps here once, on a fresh stack, with
/// no prior Rust state; it must not be called from Rust. It initialises the heap
/// before any allocation and never returns.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    // Bring the radio up as far as it will go, then serve regardless of how far
    // that was. Serving keeps the service registered so the panel can read the
    // stage; exiting or hanging here would drop the service and show as nothing.
    let (mapped, stage) = bring_up();
    serve::run(mapped, stage);
}
