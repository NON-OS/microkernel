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

//! OP_LAYER_CONTENT's body: the handle the sender's layer shows, the surface
//! drawn over it (0 for none), its corner on the layer, its flags, the box
//! of it that changed in the drawn box's pixels, the box of the surface
//! drawn (a window's part of a buffer that also holds its shadow), and a
//! zero word.

use super::area::Area;
use super::parts::{parts, Parts};
use super::read::{wide, words};
use crate::protocol::{LAYER_CONTENT_PARTS_REQ_LEN, LAYER_CONTENT_REQ_LEN};

/// The flags word's opaque bit: every pixel drawn opaque, its alpha unread.
pub const CONTENT_OPAQUE: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContentSet {
    pub on: u64,
    pub handle: u64,
    pub x: u32,
    pub y: u32,
    pub opaque: bool,
    pub area: Area,
    /// The box of the surface drawn from the corner, in its pixels.
    pub view: Area,
}

/// None for a body of another length, no layer named, a flag no bit names,
/// a padding word not zero, or a layer named its own content.
pub fn layer_content(body: &[u8]) -> Option<ContentSet> {
    let [on_lo, on_hi, lo, hi, x, y, flags, ax, ay, aw, ah, vx, vy, vw, vh, pad] =
        words::<16, LAYER_CONTENT_REQ_LEN>(body)?;
    let (on, handle) = (wide(on_lo, on_hi), wide(lo, hi));
    if on == 0 || on == handle || flags & !CONTENT_OPAQUE != 0 || pad != 0 {
        return None;
    }
    let area = Area { x: ax, y: ay, width: aw, height: ah };
    let view = Area { x: vx, y: vy, width: vw, height: vh };
    Some(ContentSet { on, handle, x, y, opaque: flags & CONTENT_OPAQUE != 0, area, view })
}

/// OP_LAYER_CONTENT_PARTS' body: a layer's content, then a parts block of
/// boxes inside the box of it that changed.
pub fn layer_content_parts(body: &[u8]) -> Option<(ContentSet, Parts)> {
    if body.len() != LAYER_CONTENT_PARTS_REQ_LEN {
        return None;
    }
    let (head, block) = body.split_at(LAYER_CONTENT_REQ_LEN);
    let set = layer_content(head)?;
    Some((set, parts(block, set.area)?))
}
