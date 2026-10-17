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

//! The claim on the boot framebuffer decides who draws on it: while a driver
//! holds it the kernel stops, and when it is given back, or its holder exits,
//! the kernel draws again (L31), unless a GPU driver's claim took its
//! scanout, after which nobody but that driver draws on it (L79).

extern crate alloc;

use alloc::vec::Vec;
use spin::MutexGuard;

use super::register::is_boot_framebuffer;
use crate::hardware::broker::claim::Claim;
use crate::kernel_core::init::framebuffer::{set_held, taken, wait_quiet};
use crate::sys::serial;

/// A claim made ready in the claims' hold `claims`. The boot framebuffer's
/// hold is set under it; once it is let go, a draw already under way ends
/// before the holder's first picture (L79), and the line is said: a serial
/// line takes milliseconds, and the shutdown's quiesce waits only one (L77).
pub(in crate::hardware::broker) fn admitted(claims: MutexGuard<'static, Vec<Claim>>, id: u64) {
    if is_boot_framebuffer(id) {
        set_held(true);
        drop(claims);
        wait_quiet();
        say(true);
    }
}

pub(in crate::hardware::broker) fn released(device_id: u64) -> bool {
    let fb = is_boot_framebuffer(device_id);
    if fb {
        set_held(false);
    }
    fb
}

pub(in crate::hardware::broker) fn say(held: bool) {
    let line: &[u8] = if held {
        b"[BROKER] boot framebuffer claimed: the kernel stops drawing on it"
    } else if taken() {
        b"[BROKER] boot framebuffer let go: a display driver takes its scanout, and the kernel \
          does not draw on it again"
    } else {
        b"[BROKER] boot framebuffer given back: the kernel draws on it again"
    };
    serial::println(line);
}
