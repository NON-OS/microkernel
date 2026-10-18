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
//! driver.usb_mic0: a USB Audio 1.0 or 2.0 microphone on any USB host's
//! port, root or virtual, heard only by a peer that holds
//! MicrophoneCapture. The kernel never sees a microphone, so this driver
//! is the gate (gate.rs), and the compositor is told whenever capture
//! starts and stops (notify.rs). The class code is driver.usb_audio0's,
//! and the batch reader and indicator notice driver.uvc0's, mounted here.

#![no_std]
#![no_main]

extern crate alloc;

#[path = "../../capsule_driver_uvc/src/batch.rs"]
mod batch;
mod begin;
mod bind;
mod device;
mod frame;
mod gate;
mod gone;
#[path = "../../capsule_driver_uvc/src/indicator.rs"]
mod indicator;
mod notify;
mod ops;
#[path = "../../capsule_driver_usb_audio/src/rate.rs"]
mod rate;
mod reads;
mod ring;
mod run;
mod start;
mod stream;
mod uac;
mod wire;
mod xhci;

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
