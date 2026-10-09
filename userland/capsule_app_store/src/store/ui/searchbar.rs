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

//! The search field, in the head band.

use nonos_app_skeleton::PaintBuffer;

use crate::store::search::Search;
use crate::store::theme::{ACCENT, CARD_BG, MUTED, TITLE};

use super::metrics::{HEAD_H, PAD_TOP, SMALL_PX};
use super::text;

const W: u32 = 260;
const H: u32 = 26;
const PAD: u32 = 10;

/// Paints the field and reports how much width it took, so the head
/// can put its count to the left of it rather than underneath.
pub fn paint(fb: &mut PaintBuffer, search: &Search, right: u32) -> u32 {
    if !search.active && search.text().is_empty() {
        return 0;
    }
    let x = right.saturating_sub(W);
    let y = PAD_TOP + (HEAD_H - H) / 2;
    fb.fill_rect(x, y, W, H, CARD_BG);
    if search.active {
        fb.fill_rect(x, y + H - 2, W, 2, ACCENT);
    }
    let top = text::top_of(y as i32, H, SMALL_PX);
    match search.text().is_empty() {
        true => text::line(fb, x + PAD, top, b"type to search", MUTED, SMALL_PX),
        false => text::line(fb, x + PAD, top, search.text(), TITLE, SMALL_PX),
    };
    if search.active {
        let caret = x + PAD + text::width_of(search.text(), SMALL_PX) + 2;
        fb.fill_rect(caret.min(x + W - 3), y + 5, 1, H - 10, ACCENT);
    }
    W
}
