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
mod firmware;
mod geometry;
mod libc_firmware;
mod record;
mod serve;

use nonos_gfx_pool::Pool;
use nonos_gfx_serve::Server;
use nonos_libc::{mk_exit, start_driver};

use crate::display::BootFb;

const DRIVER: &[u8] = b"driver.bootfb";

/// With no boot framebuffer listed (another driver's machine, or firmware
/// left none) the driver says so and leaves (`EXIT_ABSENT`). One that is
/// listed is claimed on the shared bounded schedule; from then on the
/// kernel draws nothing on it and the driver serves it as driver.bootfb0.
///
/// # Safety
/// The kernel enters here once, on the capsule's own stack.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let up = start_driver(DRIVER, discover::find(), |r| {
        let fb = bring_up::bring_up(&mut libc_firmware::Libc, r).map_err(why)?;
        Ok(Server::new(BootFb { fb, fence: serve::fence }, Pool))
    });
    match up {
        Ok(mut server) => serve::run(&mut server),
        Err(code) => mk_exit(code),
    }
}

fn why(w: firmware::Why) -> &'static str {
    use firmware::Why::*;
    match w {
        Record => "not the boot framebuffer",
        Announce => "driver.bootfb0 not registered",
        Claim => "claim refused",
        Geometry => "mode not given",
        Format => "three-byte pixels",
        Map => "bar 0 not mapped",
        Shape => "mode outside the range",
    }
}
