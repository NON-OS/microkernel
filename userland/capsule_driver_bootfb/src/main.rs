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

mod bring_up;
mod copy_in;
mod describe;
mod discover;
mod display;
mod fb;
mod fb_wipe;
mod firmware;
mod geometry;
mod libc_firmware;
mod libc_scanout;
mod loss;
mod record;
mod serve;
mod start;

use nonos_gfx_pool::Pool;
use nonos_gfx_serve::Server;
use nonos_libc::{heap_init, mk_exit, start_driver};

use crate::display::BootFb;

const DRIVER: &[u8] = b"driver.bootfb";
/// No heap: the display server allocates from the first request on.
const EXIT_HEAP_INIT: i32 = 1;

/// With no boot framebuffer listed (another driver's machine, or firmware
/// left none) the driver says so and leaves (`EXIT_ABSENT`). One that is
/// listed is claimed on the shared bounded schedule; from then on the
/// kernel draws nothing on it and the driver serves it as driver.bootfb0.
///
/// # Safety
/// The kernel enters here once, on the capsule's own stack.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(EXIT_HEAP_INIT);
    }
    let up = start_driver(DRIVER, discover::find(), |r| {
        let fb = match bring_up::bring_up(&mut libc_firmware::Libc, r) {
            Err(firmware::Why::Refused) => start::refused(),
            got => got.map_err(start::why)?,
        };
        let (holds, release) = (libc_scanout::holds, libc_scanout::release);
        let device_id = r.device_id;
        let display =
            BootFb { fb, fence: serve::fence, device_id, holds, release, gone: false, told: false };
        Ok(Server::new(display, Pool::default()))
    });
    match up {
        Ok(mut server) => serve::run(&mut server),
        Err(code) => mk_exit(code),
    }
}
