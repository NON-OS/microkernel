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

//! The home screen below its top line, in two columns or in one.

use nonos_app_skeleton::PaintBuffer;

use super::home_pills::pills;
use super::home_tiles::{coins, more, read};
use crate::wallet::etna::layout::halves;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::GAP;
use crate::wallet::state::State;

/// Lays the columns from `top` down; where they end.
pub(super) fn columns(state: &State, fb: &mut PaintBuffer, c: Rect, top: u32) -> u32 {
    /* Wide: the coins and what to do with them on the left, the account,
     * its network and the rest on the right. Narrow: one column. */
    if crate::wallet::etna::current::now().two_up {
        let ((lx, lw), (rx, rw)) = halves(c.x, c.w, 2 * GAP);
        let left = Rect::new(lx, top, lw, c.h);
        let right = Rect::new(rx, top, rw, c.h);
        let mut ly = top + coins(state, fb, left, top);
        ly += read(state, fb, left, ly) + GAP;
        ly += super::home_actions::actions(state, fb, left, ly);
        let mut ry = top + pills(state, fb, right, top) + GAP;
        ry += more(fb, right, ry);
        ly.max(ry)
    } else {
        let mut y = top;
        y += pills(state, fb, c, y) + GAP;
        y += coins(state, fb, c, y);
        y += read(state, fb, c, y) + GAP;
        y += super::home_actions::actions(state, fb, c, y) + GAP;
        y + more(fb, c, y)
    }
}
