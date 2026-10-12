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

//! The framebuffer as the kernel hands it over: the boot handoff's
//! `FramebufferInfo` (`src/boot/handoff/types/framebuffer.rs`, repr(C),
//! little-endian): ptr u64, size u64, then width, height, stride (bytes),
//! pixel_format, cursor_y and phys_mm u32, 40 bytes. On x86_64 UEFI GOP
//! fills it; on aarch64 the kernel fills the same struct from a
//! simple-framebuffer node or the Pi firmware's mailbox (C-16), so one
//! decode serves every lane and the capsule never parses firmware itself.

use super::fb::Layout;

pub const INFO_LEN: usize = 40;

/// The handoff's pixel_format codes (`boot::handoff::types::constants`).
pub const RGB: u32 = 0;
pub const BGR: u32 = 1;
pub const RGBX: u32 = 2;
pub const BGRX: u32 = 3;

/// What `Fb::new` is given once the range is mapped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Record {
    /// Physical base and length of the range to map.
    pub phys: u64,
    pub len: u64,
    pub size: (u32, u32),
    pub stride: u32,
    pub layout: Layout,
    /// w | h << 16 mm, 0 unknown, as the wire's CAPS carries it.
    pub physical_mm: u32,
}

/// The 32-bit layouts the wire can show; three-byte pixels are refused.
pub fn layout_of(pixel_format: u32) -> Option<Layout> {
    match pixel_format {
        BGRX => Some(Layout::BlueFirst),
        RGBX => Some(Layout::RedFirst),
        _ => None,
    }
}

/// None for a short record, no framebuffer (a zero base or size), or a
/// layout the wire cannot show; `Fb::new` checks the geometry again
/// against what was mapped.
pub fn decode(b: &[u8]) -> Option<Record> {
    let b = b.get(..INFO_LEN)?;
    let q = |i: usize| u64::from_le_bytes(b[i..i + 8].try_into().unwrap_or([0; 8]));
    let d = |i: usize| u32::from_le_bytes(b[i..i + 4].try_into().unwrap_or([0; 4]));
    let (phys, len, size, stride) = (q(0), q(8), (d(16), d(20)), d(24));
    let layout = layout_of(d(28))?;
    let physical_mm = d(36);
    let given = phys != 0 && len != 0 && size.0 != 0 && size.1 != 0 && stride != 0;
    given.then_some(Record { phys, len, size, stride, layout, physical_mm })
}
