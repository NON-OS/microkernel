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

//! Where the splash's text sits, shared by the painter and the damage it
//! sends: the title above the middle, the attestation panel centred a little
//! above it, the warning's row and the status line under it. Nothing of the
//! firmware's screen is kept: the splash paints the whole canvas.

pub const PANEL_W: u32 = 380;
pub const PANEL_H: u32 = 112;
pub const STATUS_ROWS: u32 = 16;
/// The panel's bottom to the no-IOMMU warning, a row of the status line's
/// height kept in the gap above the status line.
pub const WARN_GAP: u32 = 14;

/// (x, y, width, height) on a `w` x `h` surface.
pub type Area = (u32, u32, u32, u32);

pub fn panel(w: u32, h: u32) -> Area {
    (w.saturating_sub(PANEL_W) / 2, (h / 2).saturating_sub(24), PANEL_W, PANEL_H)
}

pub fn status(w: u32, h: u32) -> Area {
    (0, h / 2 + 132, w, STATUS_ROWS)
}

/// The warning's row for a panel at `p` on a surface `w` wide. It is kept
/// whether or not the warning is drawn, so the layout does not depend on it.
pub fn warning(p: Area, w: u32) -> Area {
    (0, p.1.saturating_add(p.3).saturating_add(WARN_GAP), w, STATUS_ROWS)
}
