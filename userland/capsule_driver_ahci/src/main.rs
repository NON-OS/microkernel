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

mod choose;
mod clock;
mod constants;
mod controller;
mod discover;
mod engine;
mod error;
mod exit_code;
mod handles;
mod identity;
mod log;
mod protocol;
mod regs;
mod server;
mod setup;

use alloc::vec::Vec;

use nonos_libc::{heap_init, mk_exit, start_driver};

const DRIVER: &[u8] = b"driver.ahci";

/// # Safety
/// The capsule entry point. The kernel loader calls this once on a fresh stack
/// with the capsule's heap region reserved; it must never be called from Rust.
///
/// Without an AHCI controller the driver says so and leaves (`EXIT_ABSENT`)
/// before claiming anything. With one, the bring-up (`setup::run`) is
/// retried on the shared bounded schedule while no disk comes up; running
/// out with no disk on any port is `EXIT_EMPTY`, any other way `EXIT_GAVE_UP`.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }

    let hosts = discover::find_ahci();
    let present = (!hosts.is_empty()).then_some(hosts);
    let mut last = None;
    let started = start_driver(DRIVER, present, |h: &Vec<_>| {
        setup::run(h).map_err(|e| {
            last = Some(e);
            error::reason(e)
        })
    });
    let mut driver = match started {
        Ok(driver) => driver,
        Err(code) => mk_exit(exit_code::exit_code(code, last)),
    };

    server::run(&mut driver);
}
