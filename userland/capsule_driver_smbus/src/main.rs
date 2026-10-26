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

mod discover;
mod errno;
mod hostc;
mod pio;
mod ports;
mod regs;
mod say;
mod server;
mod setup;
mod survey;
mod wire;
mod xfer;

use nonos_libc::{mk_exit, start_driver};

const DRIVER: &[u8] = b"driver.smbus";

/// Without an Intel SMBus host the driver says so and leaves (`EXIT_ABSENT`)
/// before claiming anything. A host that is there is brought up on the shared
/// bounded schedule, its memory modules' SPD looked at once, and then its
/// transfers served on driver.smbus0.
///
/// # Safety
/// The kernel enters here once, on the capsule's own fresh stack.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    match start_driver(DRIVER, discover::find_smbus(), |dev| setup::run(*dev)) {
        Ok(mut driver) => {
            survey::survey(&mut driver);
            server::run(driver)
        }
        Err(code) => mk_exit(code),
    }
}
