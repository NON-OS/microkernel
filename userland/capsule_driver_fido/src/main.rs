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
//! driver.fido0: a FIDO2 security key on CTAPHID, on any USB host's port,
//! root or virtual. One client at a time holds the key and passes it CTAP2
//! requests, unread; the key's user, not this driver, says yes. Its
//! endpoint is held to no one until the owner names who may ask.

#![no_std]
#![no_main]

extern crate alloc;

#[path = "../../capsule_driver_usb_hid/src/descriptors/application.rs"]
mod application;
mod bind;
mod ctaphid;
mod device;
#[path = "../../capsule_driver_cdc_acm/src/frame.rs"]
mod frame;
mod gone;
mod ops;
#[path = "../../capsule_driver_cdc_acm/src/acm/owner.rs"]
mod owner;
mod pump;
mod request;
mod run;
mod session;
mod up;
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
