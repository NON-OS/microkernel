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

//! Where the shared table puts its rows, measured from its top left, for the
//! visible slice.

use nonos_toolkit::design::scale::scale;

use super::super::metrics::ROW_H;

/// How tall the header and each row are, in drawing pixels.
pub fn row_h() -> u32 {
    scale().px(ROW_H)
}

/// The header and the hairline under it, where the first row starts.
pub fn head_h() -> u32 {
    row_h().saturating_add(scale().hair())
}

pub fn row_y(index: usize) -> u32 {
    head_h() + index as u32 * row_h()
}

pub fn visible_rows(table_h: u32) -> usize {
    (table_h.saturating_sub(head_h()) / row_h().max(1)).max(1) as usize
}
