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
mod init;
mod protocol;
mod queue;
mod regs;
mod report;
mod server;
mod setup;

use nonos_libc::{heap_init, mk_exit, start_driver};

const DRIVER: &[u8] = b"driver.e1000";
const EXIT_HEAP_INIT: i32 = 1;

/// Without a card the driver says so and leaves (`EXIT_ABSENT`) before
/// claiming anything. With one, each attempt takes the grants and programs
/// the part, giving every grant back when either half fails; the attempts
/// are bounded and slept between, and running out is `EXIT_GAVE_UP`.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(EXIT_HEAP_INIT);
    }
    let started =
        start_driver(DRIVER, discover::find_e1000(), |dev| setup::run(*dev).and_then(init::finish));
    let mut driver = match started {
        Ok(d) => d,
        Err(code) => mk_exit(code),
    };
    server::run(&mut driver);
}
