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

//! Where the splash's text sits, shared by the painter, the damage it sends
//! and the logo's check: the attestation panel centred a little above the
//! middle, the status line under it. The firmware's logo stays where the
//! firmware drew it. Centred on its own screen, as firmware commonly draws
//! it, it would sit on the panel, so the panel and the status line move
//! below it while they still fit; where they do not, the logo is left out
//! and the title stands in its place.

use crate::logo::place::Spot;

pub const PANEL_W: u32 = 380;
pub const PANEL_H: u32 = 112;
pub const STATUS_ROWS: u32 = 16;
/// Below the logo's bottom edge to the panel; the panel's bottom to the
/// status line, as in the plain layout; the status line to the screen's foot.
pub const GAP: u32 = 24;
pub const STATUS_GAP: u32 = 44;
pub const FOOT: u32 = 8;

/// (x, y, width, height) on a `w` x `h` surface.
pub type Area = (u32, u32, u32, u32);

pub fn panel(w: u32, h: u32) -> Area {
    (w.saturating_sub(PANEL_W) / 2, (h / 2).saturating_sub(24), PANEL_W, PANEL_H)
}

pub fn status(w: u32, h: u32) -> Area {
    (0, h / 2 + 132, w, STATUS_ROWS)
}

fn overlaps(s: &Spot, (x, y, w, h): Area) -> bool {
    let (sx, sy) = (s.x, s.y);
    let (sr, sb) = (sx + i64::from(s.w), sy + i64::from(s.h));
    sx < i64::from(x + w) && sr > i64::from(x) && sy < i64::from(y + h) && sb > i64::from(y)
}

/// Whether the logo at `spot` leaves the panel and the status line clear.
pub fn logo_clear(spot: &Spot, w: u32, h: u32) -> bool {
    !overlaps(spot, panel(w, h)) && !overlaps(spot, status(w, h))
}
