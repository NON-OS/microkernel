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

//! Right-hand pane geometry: the content column, the document card beneath
//! the header, its rows and its "View all" button. The painter lays every
//! control out in the screen's controls from these boxes.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::widgets::{Button, Card, Kind, Rect, Row};

use super::metrics::{lh, pane_x, BODY, HEAD, PANE_PAD};

pub(super) fn pane_content(w: u32) -> (u32, u32) {
    let x = pane_x() + PANE_PAD;
    (x, w.saturating_sub(x + PANE_PAD))
}

/// Top of the document card, below the title and its line.
pub(super) fn cols_y() -> u32 {
    PANE_PAD + lh(HEAD) + 4 + lh(BODY) + 30
}

/// One document row: the name over its store path.
pub(super) fn doc_row_h() -> u32 {
    Row::new("").note("").size().1
}

/// The card titled `title` for `count` documents in a `w` by `h` window, and
/// how many rows fit in it. An empty list keeps one row for the line that
/// says so.
pub(super) fn list_card(w: u32, h: u32, title: &str, count: usize) -> (Rect, usize) {
    let ((x, cw), y) = (pane_content(w), cols_y());
    let (head, pad) = (Card::new().title(title).size().1, scale().px(u32::from(SPACE.lg)));
    let room = h.saturating_sub(y + head + pad) / doc_row_h().max(1);
    let shown = count.min(room as usize);
    let h = head + pad + doc_row_h() * shown.max(1) as u32;
    (Rect { x, y, w: cw, h }, shown)
}

/// Row `i` of a card whose rows start at the top of `content`.
pub(super) fn doc_row(content: Rect, i: usize) -> Rect {
    Rect { y: content.y + doc_row_h() * i as u32, h: doc_row_h(), ..content }
}

/// The "View all" button, right-aligned in the card's head.
pub(super) fn view_all(card: Rect, title: &str) -> (Button<'static>, Rect) {
    let button = Button::new("View all").kind(Kind::Plain);
    let ((bw, bh), head) = (button.size(), Card::new().title(title).size().1);
    let pad = scale().px(u32::from(SPACE.sm));
    let x = (card.x + card.w).saturating_sub(bw + pad);
    (button, Rect { x, y: card.y + head.saturating_sub(bh) / 2, w: bw, h: bh })
}
