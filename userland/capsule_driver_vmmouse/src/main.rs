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

//! NONOS userland: the VMware absolute pointer (vmmouse), through the
//! backdoor the kernel allows for it alone. It serves nothing: it posts
//! the host pointer's position, buttons and wheel onto the kernel input
//! ring.

#![no_std]
#![no_main]

mod backdoor;
mod constants;
mod packet;
mod post;
mod say;
mod serve;
mod setup;

use nonos_libc::{bring_up, mk_exit, say_absent, EXIT_ABSENT, EXIT_GAVE_UP};

use post::Pointer;
use serve::Driver;

const DRIVER: &[u8] = b"driver.vmmouse";

/// With no backdoor the driver says so and leaves. With one, enabling the
/// pointer is tried on the shared bounded schedule.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let Some(version) = setup::version() else {
        say_absent(DRIVER);
        mk_exit(EXIT_ABSENT);
    };
    if bring_up(DRIVER, setup::enable).is_err() {
        mk_exit(EXIT_GAVE_UP);
    }
    say::up(version);
    serve::run(&mut Driver { pointer: Pointer::default(), lost: false });
}
