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

//! Where a section's card sits and where each of its rows falls. Everything
//! is derived from the content width and the toolkit's own sizes, so the card
//! grows with the scale and its rows with it.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::widgets::{Card, Rect, Row};

use super::geom::{head_top, pane_x, PANE_PAD};

/// The height of one row of a card.
pub(super) fn row_h() -> u32 {
    Row::new("").size().1
}

/// The card for a section headed `head` with `rows` rows, across the pane.
pub(super) fn card_rect(width: u32, head: &str, rows: usize) -> Rect {
    let x = pane_x() + PANE_PAD;
    let pad = scale().px(u32::from(SPACE.lg));
    let h = Card::new().title(head).size().1 + row_h() * rows as u32 + pad;
    Rect { x, y: head_top(), w: width.saturating_sub(x + PANE_PAD).max(1), h }
}

/// Row `i` of a card whose rows start at the top of `content`.
pub(super) fn row_rect(content: Rect, i: usize) -> Rect {
    Rect { y: content.y + row_h() * i as u32, h: row_h(), ..content }
}
