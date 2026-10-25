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

//! One axis of a popup's place. Pure, so the host proofs hold it. The
//! anchor names a point on the anchor rectangle; the popup grows from it
//! the way gravity says and moves by the offset. Then it is kept inside the
//! bounds as far as its constraint adjustments allow, in the protocol's
//! order: flipped to the other side if that fits, slid along, cut down.

/// Where the anchor rectangle starts and its length, the side the anchor
/// and the gravity name (-1, 0 or 1), the popup's length and its offset.
#[derive(Clone, Copy)]
pub struct Axis {
    pub start: i64,
    pub len: i64,
    pub anchor: i8,
    pub gravity: i8,
    pub size: i64,
    pub offset: i64,
}

fn origin(a: Axis) -> i64 {
    let point = match a.anchor {
        -1 => a.start,
        1 => a.start + a.len,
        _ => a.start + a.len / 2,
    };
    let grown = match a.gravity {
        -1 => a.size,
        1 => 0,
        _ => a.size / 2,
    };
    point + a.offset - grown
}

/// The popup's start and length on this axis, kept within `lo..hi` as
/// flip, slide and resize allow.
pub fn fit(a: Axis, (lo, hi): (i64, i64), (flip, slide, resize): (bool, bool, bool)) -> (i64, i64) {
    let inside = |at: i64, size: i64| at >= lo && at + size <= hi;
    let (mut at, mut size) = (origin(a), a.size);
    let other = origin(Axis { anchor: -a.anchor, gravity: -a.gravity, offset: -a.offset, ..a });
    if flip && !inside(at, size) && inside(other, size) {
        at = other;
    }
    if slide && !inside(at, size) {
        at = at.min(hi - size).max(lo);
    }
    let (from, to) = (at.max(lo), (at + size).min(hi));
    if resize && !inside(at, size) && to > from {
        (at, size) = (from, to - from);
    }
    (at, size)
}
