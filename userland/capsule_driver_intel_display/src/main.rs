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

mod bar;
mod clock;
mod discover;
mod dma_pair;
mod error;
mod frames;
mod frames_drop;
mod gone;
mod handles;
mod senders;
mod serve;
mod setup;
mod turn;

use nonos_libc::{heap_init, mk_exit, start_driver};
// `turn` reaches the DGON parser here, where its proofs mount the kernel's
// own definition under the same name.
use nonos_libc::dgon_parse;

use crate::error::reason;

const DRIVER: &[u8] = b"driver.intel_display";

/// Without a covered Intel graphics function the driver says so and leaves
/// (`EXIT_ABSENT`) before claiming anything. One that is there is taken
/// over on the shared bounded schedule, and running out is `EXIT_GAVE_UP`,
/// with the display as firmware left it. Once up, the driver serves the
/// display wire as `driver.intel_display0` until it is stopped.
///
/// # Safety
/// The kernel enters here once, on the capsule's own fresh stack.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    let started = start_driver(DRIVER, discover::find(), |dev| setup::run(*dev).map_err(reason));
    let mut driver = match started {
        Ok(driver) => driver,
        Err(code) => mk_exit(code),
    };
    let device_id = driver.handles.device_id;
    serve::run(&mut driver.server, device_id)
}
