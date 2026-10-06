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
#![no_std]
#![no_main]
extern crate alloc;
mod constants;
mod contexts;
mod controller;
mod discover;
mod dma;
mod error;
mod handles;
mod protocol;
mod regs;
mod rings;
mod server;
mod setup;
mod slots;
mod trb;
use crate::error::reason;
use nonos_libc::{heap_init, mk_exit, start_driver};
const DRIVER: &[u8] = b"driver.xhci";
/// # Safety
/// The capsule entry point. The kernel loader calls this once on a fresh stack
/// with the capsule's heap region reserved; it must never be called from Rust.
///
/// Without a controller the driver says so and leaves (`EXIT_ABSENT`) before
/// claiming anything. A controller that is there is brought up on the shared
/// bounded schedule, each failed attempt releasing what it took, and running
/// out is `EXIT_GAVE_UP`.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    let all = discover::find_all_xhci();
    let started =
        start_driver(DRIVER, all.first().copied(), |dev| setup::run(*dev).map_err(reason));
    let mut drivers = match started {
        Ok(d) => alloc::vec![d],
        Err(code) => mk_exit(code),
    };
    // The other controllers are best effort: one that fails (a Thunderbolt
    // controller powered down with nothing attached, say) leaves the
    // primary's ports served.
    for dev in all.iter().skip(1) {
        if let Ok(d) = setup::run(*dev) {
            drivers.push(d);
        }
    }
    server::run(drivers);
}
