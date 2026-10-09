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

//! How far down the list you are.

use nonos_app_skeleton::PaintBuffer;

use crate::store::theme::{CARD_SEL_EDGE, RULE};

use super::metrics::{CARD_GAP, CARD_H};

const W: u32 = 3;
const GAP: u32 = 6;

pub fn paint(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, rows: usize, total: usize, at: usize) {
    if total <= rows || rows == 0 {
        return;
    }
    let track_h = rows as u32 * (CARD_H + CARD_GAP) - CARD_GAP;
    let left = x + w + GAP;
    fb.fill_rect(left, y, W, track_h, RULE);
    /*
     * The thumb is the fraction of the list in view, never thinner than it can
     * be seen: a two hundred entry catalogue would otherwise round it away to
     * nothing at the very moment it is most wanted.
     */
    let span = (track_h as usize * rows / total).max(12) as u32;
    let travel = track_h.saturating_sub(span);
    let most = total.saturating_sub(rows);
    let top = y + (travel as usize * at.min(most) / most.max(1)) as u32;
    fb.fill_rect(left, top, W, span, CARD_SEL_EDGE);
}
