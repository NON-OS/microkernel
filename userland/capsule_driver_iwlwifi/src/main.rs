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
mod discover;
mod driver;
mod firmware;
mod init;
mod mmio;
// Which PCI function is a supported adapter, decided without the INTx line.
mod pci_match;
mod regs;
mod server;
mod setup;

use nonos_libc::{heap_init, mk_exit, start_driver};

const DRIVER: &[u8] = b"driver.iwlwifi";

/// # Safety
/// The capsule entry point. The kernel loader calls this once on a fresh stack
/// with the capsule's heap region reserved; it must never be called from Rust.
///
/// Without a supported adapter the driver says so and leaves (`EXIT_ABSENT`)
/// before claiming anything. An adapter that is there is brought up on the
/// shared bounded schedule, each failed attempt giving back every grant, and
/// running out is `EXIT_GAVE_UP`.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    let driver = match start_driver(DRIVER, discover::find_iwlwifi(), |dev| setup::run(*dev)) {
        Ok(driver) => driver,
        Err(code) => mk_exit(code),
    };
    server::run(driver);
}
