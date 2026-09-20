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

//! Where the list is.

use super::metrics::{
    ACTION_H, ACTION_W, CARD_GAP, CARD_H, CARD_PAD, DETAIL_W, HEAD_H, PAD_TOP, PAD_X, TAB_H,
    TAB_TO_LIST,
};

/// The y the first card starts at.
pub fn list_top() -> u32 {
    PAD_TOP + HEAD_H + TAB_H + TAB_TO_LIST
}

/// How wide the list is, given the surface. The detail pane and three
/// gutters take the rest.
pub fn list_w(fb_w: u32) -> u32 {
    fb_w.saturating_sub(PAD_X * 3 + DETAIL_W)
}

pub fn row_top(slot: usize) -> u32 {
    list_top() + slot as u32 * (CARD_H + CARD_GAP)
}

/// Which visible slot a point falls in.
pub fn slot_at(y: i32, rows: usize) -> Option<usize> {
    let top = list_top() as i32;
    if y < top {
        return None;
    }
    let pitch = (CARD_H + CARD_GAP) as i32;
    let slot = (y - top) / pitch;
    let within = (y - top) % pitch;
    match within < CARD_H as i32 && (slot as usize) < rows {
        true => Some(slot as usize),
        false => None,
    }
}

/// The action control inside a card whose box is `x, top, w`.
pub fn action_rect(x: u32, top: u32, w: u32) -> (u32, u32, u32, u32) {
    let ax = x + w.saturating_sub(CARD_PAD + ACTION_W);
    let ay = top + (CARD_H - ACTION_H) / 2;
    (ax, ay, ACTION_W, ACTION_H)
}

/// Whether a point is inside that control.
pub fn on_action(x: i32, y: i32, card_x: u32, top: u32, w: u32) -> bool {
    let (ax, ay, aw, ah) = action_rect(card_x, top, w);
    x >= ax as i32 && x < (ax + aw) as i32 && y >= ay as i32 && y < (ay + ah) as i32
}
