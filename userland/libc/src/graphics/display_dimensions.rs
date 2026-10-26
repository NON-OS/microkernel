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

use crate::syscall::{call_raw, N_GFX_DISPLAY_DIMENSIONS};

#[no_mangle]
pub extern "C" fn nonos_display_dimensions(
    display: u32,
    out_width: *mut u32,
    out_height: *mut u32,
) -> i64 {
    if out_width.is_null() || out_height.is_null() {
        return -22;
    }
    call_raw(
        N_GFX_DISPLAY_DIMENSIONS,
        [display as u64, out_width as u64, out_height as u64, 0, 0, 0],
    )
}

/// The panel's physical size from its EDID, as the kernel had it from the
/// bootloader: width in millimetres in the low 16 bits, height in the high
/// 16. Writes 0 when the firmware gave no size. Returns the syscall result.
#[no_mangle]
pub extern "C" fn nonos_display_physical_mm(display: u32, out_mm: *mut u32) -> i64 {
    if out_mm.is_null() {
        return -22;
    }
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    call_raw(
        N_GFX_DISPLAY_DIMENSIONS,
        [
            display as u64,
            &mut width as *mut u32 as u64,
            &mut height as *mut u32 as u64,
            out_mm as u64,
            0,
            0,
        ],
    )
}

/// Why there is no boot framebuffer, as the kernel had it from the loader:
/// bits 0 to 7 the reason, 0 no Graphics Output Protocol, 1 PixelBltOnly, 2
/// only PixelBitMask modes NONOS cannot draw (bits 8 to 15 their bits per
/// pixel), 3 a framebuffer that does not cover its mode, 0xFD one the kernel
/// would not map, 0xFE a reason the kernel did not trust, 0xFF none given.
/// Returns -95 (ENOTSUP) with that word written, or 0 with 0 written when
/// there is a framebuffer.
#[no_mangle]
pub extern "C" fn nonos_display_refusal(display: u32, out_why: *mut u32) -> i64 {
    if out_why.is_null() {
        return -22;
    }
    let (mut width, mut height) = (0u32, 0u32);
    let (w, h) = (&mut width as *mut u32 as u64, &mut height as *mut u32 as u64);
    call_raw(N_GFX_DISPLAY_DIMENSIONS, [display as u64, w, h, 0, out_why as u64, 0])
}
