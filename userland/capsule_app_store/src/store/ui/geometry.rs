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

use super::metrics::{CARD_GAP, CARD_H, DETAIL_W, HEAD_H, PAD_TOP, PAD_X, TAB_H, TAB_TO_LIST};

#[path = "geometry_controls.rs"]
mod geometry_controls;

pub use geometry_controls::{action_rect, on_action, on_remove, remove_rect};

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
