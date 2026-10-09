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

//! How wide the wallet's column is in a window of a given width, and how
//! large its type. Pure, so wallet_proofs holds the rule.
//!
//! The frame was a fixed 560 pixel column, the phones' width, centred in
//! whatever window it was given: at full screen it was a phone in a desktop
//! window. The column now takes the window less a margin, up to a width a
//! line of text stays readable at, and the type grows with it. From
//! `TWO_UP_FROM` the home screen lays its parts in two columns.

/// The section photographs' width, and the narrowest column the frame lays.
pub const PHOTO_W: u32 = 560;
/// The widest column: past it a line of text is harder to follow, not easier.
pub const MAX_COLUMN: u32 = 1040;
/// From this column width the home screen is two columns.
pub const TWO_UP_FROM: u32 = 960;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Layout {
    /// The column's left edge in the window.
    pub x: u32,
    /// The column's width.
    pub column: u32,
    /// The type's size, in percent of the desktop base.
    pub scale_pct: u32,
    /// Whether the home screen lays two columns.
    pub two_up: bool,
}

/// The widest margin either side of the column.
pub const MARGIN: u32 = 64;

/// The column's width in a window `width` wide. Up to the photographs' width
/// it is the window; past it the column takes half of each further pixel
/// until the margins reach `MARGIN`, then all of it, up to `MAX_COLUMN`. So
/// it never narrows as the window widens, and the margins open gradually.
fn column(width: u32) -> u32 {
    let eased = PHOTO_W + width.saturating_sub(PHOTO_W) / 2;
    let full = width.saturating_sub(2 * MARGIN);
    eased.min(width).max(full).min(MAX_COLUMN)
}

pub fn layout(width: u32) -> Layout {
    let column = column(width);
    let scale_pct = match column {
        1000.. => 125,
        760.. => 112,
        _ => 100,
    };
    Layout { x: (width - column) / 2, column, scale_pct, two_up: column >= TWO_UP_FROM }
}

/// Two columns `gap` apart in the span from `x`, `w` wide: the left and the
/// right, each as its left edge and width.
pub fn halves(x: u32, w: u32, gap: u32) -> ((u32, u32), (u32, u32)) {
    let left = w.saturating_sub(gap) / 2;
    let right = w.saturating_sub(gap + left);
    ((x, left), (x + left + gap, right))
}

/// Where a screen's content goes across the column: its left edge and width,
/// `side` in from each edge of the column.
pub fn content(l: &Layout, side: u32) -> (u32, u32) {
    (l.x + side, l.column.saturating_sub(2 * side))
}

/// The lowest row content may reach in a window `height` tall whose foot is
/// `foot` tall, `room` clear of it, so the last line never meets the foot or
/// the window's edge.
pub fn content_bottom(height: u32, foot: u32, room: u32) -> u32 {
    height.saturating_sub(foot + room)
}
