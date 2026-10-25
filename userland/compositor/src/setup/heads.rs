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

//! The mirrored outputs of a wire driver: SURFACE on each output past 0
//! that CAPS counts (as many as the wire server keeps, four), attached; a
//! head that does not answer is left out until the next poll asks again.
//! Giving them back releases each attachment.

use alloc::vec::Vec;

use nonos_libc::mk_surface_release;

use super::attach_primary::attach_primary;
use crate::display::{ops, Head};
use crate::sw_blitter::Surface;

/// The outputs a wire server keeps (`nonos_gfx_serve::MAX_OUTPUTS`).
const MAX_OUTPUTS: u16 = 4;

pub fn prime_heads(port: u32, outputs: u16) -> Vec<Head> {
    let last = outputs.min(MAX_OUTPUTS);
    (1..last).filter_map(|o| head(port, o)).collect()
}

pub fn head(port: u32, output: u16) -> Option<Head> {
    let s = ops::surface(port, output).ok()?;
    let p = attach_primary(s.handle).ok()?;
    let screen = Surface {
        base_va: p.va,
        stride: p.stride,
        width: p.width,
        height: p.height,
        byte_len: p.byte_len,
    };
    let (serial, handle, cursor_offset) = (s.serial, s.handle, s.cursor_offset);
    Some(Head { output, serial, handle, screen, stale: false, cursor_offset, shaped: false })
}

pub fn release_heads(heads: &mut Vec<Head>) {
    for h in heads.drain(..) {
        let _ = mk_surface_release(h.handle);
    }
}
