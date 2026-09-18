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

//! Where the home screen puts things, for the painter and the hit test both.
//!
//! These numbers lived twice: once in the painter and once in the pointer
//! handler, under a comment reading "mirror paint_home.rs". Moving a row in one
//! place therefore moved the picture without moving the click target, and the
//! quick actions silently stopped landing where they were drawn. A hit test that
//! has to be kept in sync by hand is a bug with a date on it.

use super::paint_network_card::NET_H;

pub const TOP: u32 = 146;
pub const SECTION_GAP: u32 = 24;
pub const LABEL_DROP: u32 = 26;
pub const QUICK_H: u32 = 82;
pub const QUICK_GAP: u32 = 16;
pub const LEFT: u32 = 226;
pub const SIDEBAR: u32 = 252;

/// The taller of the two cards on the top row decides where the rest begins.
pub const ROW_H: u32 = NET_H;

pub const fn actions_label() -> u32 {
    TOP + ROW_H + SECTION_GAP
}

/// Top of the quick-action cards.
pub const fn actions() -> u32 {
    actions_label() + LABEL_DROP
}

/// Caption row of the rails and activity section.
pub const fn rails() -> u32 {
    actions() + QUICK_H + SECTION_GAP
}

/// Width of one quick-action card for a given framebuffer width.
pub fn quick_w(fb_w: u32) -> u32 {
    let cw = fb_w.saturating_sub(SIDEBAR);
    cw.saturating_sub(QUICK_GAP * 3) / 4
}

/// The x of quick-action card `i`.
pub fn quick_x(fb_w: u32, i: u32) -> u32 {
    LEFT + i * (quick_w(fb_w) + QUICK_GAP)
}
