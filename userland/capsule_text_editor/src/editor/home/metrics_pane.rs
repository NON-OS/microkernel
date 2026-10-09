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

//! Right-hand pane geometry: the content column, and the document list
//! beneath the header.

use super::metrics::{lh, pane_x, BODY, HEAD, PANE_PAD, SUBHEAD};

pub(super) fn pane_content(w: u32) -> (u32, u32) {
    let x = pane_x() + PANE_PAD;
    (x, w.saturating_sub(x + PANE_PAD))
}

/// Top of the list's heading, below the title and its line.
pub(super) fn cols_y(_w: u32) -> u32 {
    PANE_PAD + lh(HEAD) + 4 + lh(BODY) + 30
}

pub(super) fn doc_row_h() -> u32 {
    lh(BODY) * 2 + 14
}

pub(super) fn docs_rect(w: u32) -> (u32, u32, u32) {
    let (x, cw) = pane_content(w);
    let y = cols_y(w) + lh(SUBHEAD) + 14;
    (x, y, cw)
}

pub(super) fn docs_list_rect(w: u32, h: u32, count: usize) -> (u32, u32, u32, u32) {
    let (x, y, cw) = docs_rect(w);
    let rh = doc_row_h().max(1);
    let room = (h.saturating_sub(y) / rh) as usize;
    (x, y, cw, count.min(room) as u32 * rh)
}
