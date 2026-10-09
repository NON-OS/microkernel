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
mod device;
mod discover;
mod driver;
mod init;
mod protocol;
mod regs;
mod server;
mod setup;
mod state;
mod virgl;
use nonos_libc::{bring_up, heap_init, mk_exit, mk_service_register, EXIT_ABSENT, EXIT_GAVE_UP};

const SERVICE_NAME: &[u8] = b"driver.virtio_gpu0";
const SERVICE_PORT: u32 = 4226;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    // The broker lists every PCI function before the first capsule starts, so
    // with no virtio-gpu a retry would only spin through boot on real
    // hardware. Leave at once (absent); the compositor takes GOP.
    if discover::find_virtio_gpu().is_none() {
        mk_exit(EXIT_ABSENT);
    }
    // A present device that fails a setup step used to be retried for ten
    // seconds with 64 yields between tries, and every try after the first
    // failed at claim because the failed one still held it: a thousand
    // logged rounds on a pegged core. The shared bring-up sleeps between a
    // bounded number of attempts, each of which releases what it claimed,
    // then gives up by name; the clean exit frees the slot and lets the
    // compositor fall back to GOP.
    let Ok(driver) = bring_up(SERVICE_NAME, setup::run) else {
        mk_exit(EXIT_GAVE_UP);
    };
    if mk_service_register(SERVICE_NAME.as_ptr(), SERVICE_NAME.len(), SERVICE_PORT) < 0 {
        mk_exit(1);
    }
    server::run(driver);
}
