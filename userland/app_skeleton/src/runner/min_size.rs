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

//! How small a window may be made. Every app lays itself out for the size it
//! opened at, and dragged far below it the layout gave out: panes drew over
//! one another and text ran across text. Each window may now be made as small
//! as three quarters of its opening size and no smaller, so every app keeps a
//! size its layout was built for. The floor never goes below the drag's own
//! minimum, and the display still bounds the window from above.

/// The smallest share of the opening size a window may be made, in percent.
pub const FLOOR_PERCENT: u32 = 75;

/// The drag's own minimum, whatever the opening size.
pub const MIN_W: u32 = 300;
pub const MIN_H: u32 = 200;

/// The smallest width and height for a window that opened at `w` by `h`.
pub fn floor(w: u32, h: u32) -> (u32, u32) {
    let share = |v: u32| (v as u64 * FLOOR_PERCENT as u64 / 100) as u32;
    (share(w).max(MIN_W), share(h).max(MIN_H))
}

/// A requested size, raised to the floor for a window that opened at
/// `opened` and then held to the display's `bound`, which wins.
pub fn settle(want: (u32, u32), opened: (u32, u32), bound: Option<(u32, u32)>) -> (u32, u32) {
    let (fw, fh) = floor(opened.0, opened.1);
    let (mut w, mut h) = (want.0.max(fw), want.1.max(fh));
    if let Some((bw, bh)) = bound {
        w = w.min(bw);
        h = h.min(bh);
    }
    (w, h)
}

/// The most a window whose origin is at `origin` can grow to on a `display`
/// sized screen: what is left right of and below the origin, since a resize
/// keeps the origin. Bounded by the whole display instead, a window dragged
/// wider near the right edge ran its right border, and the band that resizes
/// it back, off the screen, and the window manager, which keeps the window
/// on screen, then had it some pixels left of where it was drawn.
pub fn room(display: (u32, u32), origin: (u32, u32)) -> (u32, u32) {
    (display.0.saturating_sub(origin.0), display.1.saturating_sub(origin.1))
}
