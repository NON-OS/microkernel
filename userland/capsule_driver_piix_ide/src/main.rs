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

mod channel;
mod constants;
mod discover;
mod disk;
mod io;
mod regs;
mod say;
mod server;
mod setup;
mod wait;
// The whole wire as track B's crate has it, some of it for other media.
#[allow(dead_code, unused_imports)]
mod wire;

use nonos_libc::{heap_init, mk_exit, start_driver, EXIT_ABSENT};

const DRIVER: &[u8] = b"driver.piix_ide";
const EXIT_HEAP_INIT: i32 = 1;

/// Without a PIIX IDE controller the driver says so and leaves before
/// claiming anything. With one, each attempt takes the grants, resets the
/// channels and probes every slot, giving everything back when a step
/// fails; the attempts are bounded and slept between. A controller with no
/// ATA disk behind it, a CD drive alone, is given back too.
///
/// # Safety
/// The kernel enters here once, on the capsule's own fresh stack.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(EXIT_HEAP_INIT);
    }
    let mut driver = match start_driver(DRIVER, discover::find_ide(), |dev| setup::run(*dev)) {
        Ok(d) => d,
        Err(code) => mk_exit(code),
    };
    if driver.disk_count() == 0 {
        driver.release();
        say::no_disk();
        mk_exit(EXIT_ABSENT);
    }
    for (unit, d) in driver.disks.iter().enumerate() {
        if let Some(d) = d {
            say::disk(unit, d);
        }
    }
    server::run(&mut driver);
}
