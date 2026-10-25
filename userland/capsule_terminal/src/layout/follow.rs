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

//! The prompt follows the output until the body is full.
//!
//! With the prompt pinned to the window's foot, a short session showed its
//! output at the top, the prompt at the bottom and an empty band between them
//! that grew with the window: maximised it was most of the screen. The prompt
//! now sits on the row after the last line drawn, as in every other terminal,
//! and reaches the foot only once the output fills the body.

use super::types::Layout;

/// `l` with the body cut to the `used` rows the output fills and the prompt
/// moved up under them. A full body, or more, leaves the layout as it was.
pub fn follow(mut l: Layout, used: usize, row_h: u32) -> Layout {
    let rows = (l.body.h / row_h.max(1)) as usize;
    if used >= rows {
        return l;
    }
    l.body.h = used as u32 * row_h;
    l.input.y = l.body.y + l.body.h;
    l
}
