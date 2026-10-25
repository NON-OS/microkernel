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

use core::sync::atomic::{AtomicBool, Ordering};

use crate::kernel_core::surface_registry::SurfaceDescriptor;

static DISPLAY_MODE_LOGGED: AtomicBool = AtomicBool::new(false);

// The first registered surface is the GPU driver's scanout, so its geometry is
// the real display mode. Log it once so every boot proves what resolution the
// machine actually came up in.
pub(super) fn log_display_mode(desc: &SurfaceDescriptor) {
    if DISPLAY_MODE_LOGGED.swap(true, Ordering::Relaxed) {
        return;
    }
    let mut buf = [0u8; 24];
    let mut n = 0;
    n += fmt_u32(desc.width, &mut buf[n..]);
    buf[n] = b'x';
    n += 1;
    n += fmt_u32(desc.height, &mut buf[n..]);
    // serial::print, not trace: the mode must land in every boot log, not only
    // debug-enabled ones, so a wrong resolution is visible immediately.
    crate::sys::serial::print(b"[DISPLAY] mode ");
    crate::sys::serial::println(&buf[..n]);
}

fn fmt_u32(mut v: u32, out: &mut [u8]) -> usize {
    let mut tmp = [0u8; 10];
    let mut k = 0;
    loop {
        tmp[k] = b'0' + (v % 10) as u8;
        v /= 10;
        k += 1;
        if v == 0 || k == 10 {
            break;
        }
    }
    for i in 0..k {
        out[i] = tmp[k - 1 - i];
    }
    k
}
