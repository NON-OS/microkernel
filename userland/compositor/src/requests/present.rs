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

//! OP_PRESENT's body: a buffer handle, the layer's z, the buffer's index in
//! its chain, and the rectangle that changed, in the buffer's own pixels.

use super::area::Area;
use super::parts::{parts, Parts};
use super::read::{wide, words};
use crate::protocol::{PRESENT_LAYER_PARTS_REQ_LEN, PRESENT_LAYER_REQ_LEN, PRESENT_REQ_LEN};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Shown {
    pub handle: u64,
    pub z: u32,
    pub index: u32,
    pub area: Area,
    /// The client says every pixel of the buffer is opaque.
    pub opaque: bool,
}

/// The index word's opaque bit (nonos_surface::INDEX_OPAQUE).
pub const INDEX_OPAQUE: u32 = 1 << 31;

pub fn present(body: &[u8]) -> Option<Shown> {
    let [lo, hi, z, index, x, y, width, height] = words::<8, PRESENT_REQ_LEN>(body)?;
    let handle = wide(lo, hi);
    let (opaque, index) = (index & INDEX_OPAQUE != 0, index & !INDEX_OPAQUE);
    let area = Area { x, y, width, height };
    (handle != 0).then_some(Shown { handle, z, index, area, opaque })
}

/// OP_PRESENT_LAYER's body: a present, then the handle its layer shows now,
/// which names the layer among the sender's others in the band.
pub fn present_layer(body: &[u8]) -> Option<(Shown, u64)> {
    let [.., on_lo, on_hi] = words::<10, PRESENT_LAYER_REQ_LEN>(body)?;
    let shown = present(body.get(..PRESENT_REQ_LEN)?)?;
    let on = wide(on_lo, on_hi);
    (on != 0).then_some((shown, on))
}

/// OP_PRESENT_LAYER_PARTS' body: a present naming its layer, then a parts
/// block of boxes inside its changed box.
pub fn present_layer_parts(body: &[u8]) -> Option<(Shown, u64, Parts)> {
    if body.len() != PRESENT_LAYER_PARTS_REQ_LEN {
        return None;
    }
    let (head, block) = body.split_at(PRESENT_LAYER_REQ_LEN);
    let (shown, on) = present_layer(head)?;
    Some((shown, on, parts(block, shown.area)?))
}
