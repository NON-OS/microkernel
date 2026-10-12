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
mod handshake;
mod hba;
mod ioc;
mod regs;
mod setup;
mod wait;

use nonos_libc::{heap_init, mk_exit, start_driver, EXIT_ABSENT};
/// The units the setup fills, named here so the setup's files name them
/// the same way in the capsule and in its proofs.
pub use nonos_scsi_serve::Units;
use nonos_scsi_serve::{say, server};

const DRIVER: &[u8] = b"driver.mpt";
/// The service name every console line starts with.
pub const NAME: &[u8] = b"driver.mpt0";
/// endpoint.4294967359: the inbox the kernel's client drains for this
/// driver's replies, as `src/hardware/mpt_capsule` spawns it.
pub const REPLY_ENDPOINT: u64 = 4_294_967_359;
const EXIT_HEAP_INIT: i32 = 1;

/// Without a 53C1030 or SAS1068 the driver says so and leaves before
/// claiming anything. With one, each attempt maps its window and a grant,
/// brings the IOC to operational and scans the targets, giving everything
/// back when a step fails; the attempts are bounded and slept between. An
/// IOC with nothing behind it is given back too.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(EXIT_HEAP_INIT);
    }
    let mut driver = match start_driver(DRIVER, discover::find(), |dev| setup::run(*dev)) {
        Ok(d) => d,
        Err(code) => mk_exit(code),
    };
    if driver.units.count() == 0 {
        driver.release();
        say::no_disk(NAME);
        mk_exit(EXIT_ABSENT);
    }
    for (unit, d) in driver.units.disks.iter().enumerate() {
        if let Some(d) = d {
            say::unit(NAME, unit, d);
        }
    }
    server::run(&mut driver.units);
}
