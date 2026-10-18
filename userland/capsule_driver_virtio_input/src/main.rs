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
mod events;
mod init;
mod keyboard;
mod post;
mod queue;
mod say;
mod serve;
mod setup;

use alloc::vec::Vec;

use nonos_libc::{bring_up, heap_init, mk_exit, say_absent, EXIT_ABSENT, EXIT_GAVE_UP};

use constants::pci::MAX_INPUTS;

const DRIVER: &[u8] = b"driver.virtio_input";
const EXIT_HEAP_INIT: i32 = 1;

/// Every virtio-input function is brought up on its own bounded schedule;
/// one that never comes up costs only itself. With none at all the driver
/// says so and leaves before claiming anything, and with none up it gives up.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(EXIT_HEAP_INIT);
    }
    let mut found = [None; MAX_INPUTS];
    if discover::find_all(&mut found) == 0 {
        say_absent(DRIVER);
        mk_exit(EXIT_ABSENT);
    }
    let mut inputs = Vec::new();
    for dev in found.iter().flatten() {
        if let Ok(input) = bring_up(DRIVER, || setup::run(*dev)) {
            say::up(&input);
            inputs.push(input);
        }
    }
    if inputs.is_empty() {
        mk_exit(EXIT_GAVE_UP);
    }
    serve::run(&mut inputs, &mut keyboard::Keyboard::new());
}
