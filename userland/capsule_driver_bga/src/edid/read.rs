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

//! The EDID blob at BAR2+0, copied out of the window: the base block, then
//! as many extensions as it counts, up to the window's size.

use crate::constants::EDID_WINDOW;

/// The blob's bytes into `out`; their count, 0 when the window holds none.
///
/// # Safety
/// `window` must map at least `EDID_WINDOW` readable bytes.
pub unsafe fn read(window: u64, out: &mut [u8; EDID_WINDOW]) -> usize {
    let byte = |i: usize| unsafe { core::ptr::read_volatile((window as *const u8).add(i)) };
    for (i, b) in out.iter_mut().take(128).enumerate() {
        *b = byte(i);
    }
    if out[..8] != [0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0] {
        return 0;
    }
    let len = ((1 + out[126] as usize) * 128).min(EDID_WINDOW);
    for (i, b) in out.iter_mut().enumerate().take(len).skip(128) {
        *b = byte(i);
    }
    len
}
