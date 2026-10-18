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

//! A rectangle's four corners off the wire: x, y, width, height.

use super::wire::le_u32;
use crate::dispi::Rect;

pub fn rect_at(body: &[u8], at: usize) -> Option<Rect> {
    Some(Rect {
        x: le_u32(body, at)?,
        y: le_u32(body, at.checked_add(4)?)?,
        width: le_u32(body, at.checked_add(8)?)?,
        height: le_u32(body, at.checked_add(12)?)?,
    })
}
