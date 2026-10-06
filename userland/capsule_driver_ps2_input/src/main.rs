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
mod keymap;
mod mouse;
mod poll;
mod protocol;
mod ring;
mod server;
mod setup;
use nonos_libc::{heap_init, mk_exit, start_driver};

const DRIVER: &[u8] = b"driver.ps2_input";

/// # Safety
/// The capsule entry point. The kernel loader calls this once on a fresh stack
/// with the capsule's heap region reserved; it must never be called from Rust.
///
/// The broker lists the keyboard record on every machine, so presence is a
/// controller answering on its ports. A machine whose keyboard and pointer
/// are USB or i2c has none: the driver says so and leaves (`EXIT_ABSENT`)
/// at once. A controller that answers is brought up on the shared bounded
/// schedule, each failed attempt giving its claims back, and running out is
/// `EXIT_GAVE_UP`. This replaces ten seconds of retries a few yields apart,
/// which spun a core and, because a failed attempt kept its claim, could
/// never have succeeded after the first.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    let found = discover::find_ps2_kbd().filter(|dev| setup::controller_answers(*dev));
    // run() looks the keyboard record up again itself; the list is fixed.
    let driver = match start_driver(DRIVER, found, |_| setup::run()) {
        Ok(d) => d,
        Err(code) => mk_exit(code),
    };
    server::run(driver);
}
