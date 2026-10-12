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
//! driver.cdc_acm0: a USB CDC Abstract Control Model function, a serial
//! line or a modem's AT command port, on any USB host's port, root or
//! virtual. One client at a time holds the line and moves its bytes.

#![no_std]
#![no_main]

extern crate alloc;

mod acm;
mod bind;
mod control;
mod device;
mod frame;
mod gone;
mod notify;
mod ops;
mod pump;
mod run;
mod session;
mod wire;

use nonos_libc::{heap_init, mk_exit};

/// # Safety
/// The kernel enters here once, on the capsule's own fresh stack.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    run::run()
}
