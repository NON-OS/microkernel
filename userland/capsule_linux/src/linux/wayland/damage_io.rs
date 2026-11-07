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

//! A window's changed pixels read from its guest.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::super::super::super::opaque::make_opaque;
use super::super::super::super::present::source::Source;

#[path = "damage_box.rs"]
mod damage_box;
#[path = "damage_changed.rs"]
mod in_box;
pub use damage_box::{plan, Corners, Reads};
pub use in_box::changed;

/// Reads `r` of `src` from guest `pid` straight into `pic`, a picture laid
/// out as the buffer is, each to its own offset, made opaque when it is
/// XRGB, then `each` run over its rows. The bytes of `pic` between the rows
/// are neither read nor touched. The bytes read, or None when the reads do
/// not fit `pic` or the read failed, with the box's bytes in `pic` then
/// undefined.
pub fn read_in(
    pid: u32,
    src: &Source,
    r: Reads,
    pic: &mut [u8],
    each: impl Fn(&mut [u8]),
) -> Option<u64> {
    let end = r.first.checked_add(r.span()?)?;
    let at = src.at.checked_add(u64::try_from(r.first).ok()?)?;
    let part = pic.get_mut(r.first..end)?;
    let ok = match r.count {
        1 => Guest::read_into(pid, at, part),
        _ => Guest::read_rows_into(pid, at, part, (r.len, r.count, r.step)),
    };
    if !ok {
        return None;
    }
    for i in 0..r.count {
        let at = r.at(i)?;
        let px = pic.get_mut(at..at.checked_add(r.len)?)?;
        if src.opaque {
            nonos_pixel_rows::opaque_row(px);
        }
        each(px);
    }
    u64::try_from(r.bytes()).ok()
}

/// Rows `y0` up to `y1` of `src` read from guest `pid` into `buf`, opaque
/// when it is XRGB. Outside the box it said changed a client's buffer may
/// hold an older frame, so the rows are read apart and only the box is laid.
/// `buf` is the caller's to keep between frames: its capacity is reused, so
/// a steady partial frame allocates none.
pub fn rows(pid: u32, src: &Source, (y0, y1): (u32, u32), buf: &mut Vec<u8>) -> bool {
    let (w, _, stride) = src.shape;
    let from = y0 as usize * stride as usize;
    let to = (y1 as usize * stride as usize).min(src.read);
    let Some(len) = to.checked_sub(from) else { return false };
    buf.clear();
    if buf.try_reserve_exact(len).is_err() {
        return false;
    }
    buf.resize(len, 0);
    if !Guest::read_into(pid, src.at + from as u64, buf) {
        return false;
    }
    if src.opaque {
        make_opaque(buf, w, y1 - y0, stride);
    }
    true
}
