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
//! driver.uvc0: a USB Video Class camera on any USB host's port, root or
//! virtual, streamed only to a peer that holds CameraCapture. The kernel
//! never sees a camera, so this driver is the gate (gate.rs), and the
//! compositor is told whenever streaming starts and stops (notify.rs).

#![no_std]
#![no_main]

extern crate alloc;

mod batch;
mod begin;
mod bind;
mod device;
mod gate;
mod gone;
mod isoch;
mod listing;
mod notify;
mod ops;
mod run;
mod start;
mod stream;
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
