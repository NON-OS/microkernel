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

use super::metrics::metrics;

/// The y the first card starts at.
pub fn list_top() -> u32 {
    let m = metrics();
    m.pad_top + m.head_h + m.tab_h + m.tab_to_list
}

/// How wide the list is, given the surface. The detail pane and three
/// gutters take the rest.
pub fn list_w(fb_w: u32) -> u32 {
    let m = metrics();
    fb_w.saturating_sub(m.pad_x * 3 + m.detail_w)
}

pub fn row_top(slot: usize) -> u32 {
    let m = metrics();
    list_top() + slot as u32 * (m.card_h + m.card_gap)
}

/// Which visible slot a point falls in.
pub fn slot_at(y: i32, rows: usize) -> Option<usize> {
    let (top, m) = (list_top() as i32, metrics());
    if y < top {
        return None;
    }
    let pitch = (m.card_h + m.card_gap) as i32;
    let slot = (y - top) / pitch;
    let within = (y - top) % pitch;
    match within < m.card_h as i32 && (slot as usize) < rows {
        true => Some(slot as usize),
        false => None,
    }
}

/// The action control inside a card whose box is `x, top, w`.
pub fn action_rect(x: u32, top: u32, w: u32) -> (u32, u32, u32, u32) {
    let m = metrics();
    let ax = x + w.saturating_sub(m.card_pad + m.action_w);
    let ay = top + (m.card_h - m.action_h) / 2;
    (ax, ay, m.action_w, m.action_h)
}

/// The Remove control an installed card shows, left of the action.
pub fn remove_rect(x: u32, top: u32, w: u32) -> (u32, u32, u32, u32) {
    let (ax, ay, aw, ah) = action_rect(x, top, w);
    (ax.saturating_sub(aw + metrics().card_pad / 2), ay, aw, ah)
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
