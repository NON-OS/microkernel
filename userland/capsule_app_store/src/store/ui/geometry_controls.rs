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

//! Where a card's controls are, and whether a point is on one.

use super::super::metrics::{ACTION_H, ACTION_W, CARD_H, CARD_PAD};

/// The action control inside a card whose box is `x, top, w`.
pub fn action_rect(x: u32, top: u32, w: u32) -> (u32, u32, u32, u32) {
    let ax = x + w.saturating_sub(CARD_PAD + ACTION_W);
    let ay = top + (CARD_H - ACTION_H) / 2;
    (ax, ay, ACTION_W, ACTION_H)
}

/// The Remove control an installed card shows, left of the action.
pub fn remove_rect(x: u32, top: u32, w: u32) -> (u32, u32, u32, u32) {
    let (ax, ay, aw, ah) = action_rect(x, top, w);
    (ax.saturating_sub(aw + CARD_PAD / 2), ay, aw, ah)
}

/// Whether a point is inside the Remove control.
pub fn on_remove(x: i32, y: i32, card_x: u32, top: u32, w: u32) -> bool {
    let (rx, ry, rw, rh) = remove_rect(card_x, top, w);
    x >= rx as i32 && x < (rx + rw) as i32 && y >= ry as i32 && y < (ry + rh) as i32
}

/// Whether a point is inside that control.
pub fn on_action(x: i32, y: i32, card_x: u32, top: u32, w: u32) -> bool {
    let (ax, ay, aw, ah) = action_rect(card_x, top, w);
    x >= ax as i32 && x < (ax + aw) as i32 && y >= ay as i32 && y < (ay + ah) as i32
}
