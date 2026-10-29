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

//! What a commit changed, at the size its window shows it.

use alloc::vec::Vec;

use super::super::super::super::super::present::source::Source;
use super::super::super::damage::Damage;
use super::super::damage_scale::{resample_box, shown_box};
use super::rows;

type Box4 = (u32, u32, u32, u32);

/// What `said` changed of `src`, at the size it is shown: its rows from
/// the box's top, `src.shape.2` bytes a row, and the box. None when it is
/// to be shown whole, Some(None) when nothing changed.
pub fn changed(pid: u32, src: &Source, said: Damage) -> Option<Option<(Vec<u8>, Box4)>> {
    let (shape, bytes) = src.raw.unwrap_or((src.shape, src.bytes));
    let (w, h, stride) = shape;
    let b = match said.on(w, h) {
        Damage::None => return Some(None),
        Damage::Rect(x0, y0, x1, y1) => (x0, y0, x1, y1),
        Damage::Whole => return None,
    };
    let rows = rows(pid, &Source { shape, bytes, raw: None, ..*src }, (b.1, b.3))?;
    if src.raw.is_none() {
        return Some(Some((rows, b)));
    }
    let to = (src.shape.0, src.shape.1);
    let s = shown_box(b, (w, h), to);
    Some(Some((resample_box(&rows, (stride, b.1), (w, h), to, s)?, s)))
}
