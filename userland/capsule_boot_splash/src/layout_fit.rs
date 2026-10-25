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

//! The layout one splash keeps for its whole life, worked out once its
//! logo is known (`layout.rs` says why it moves).

use crate::layout::{
    logo_clear, panel, status, warning, Area, FOOT, GAP, PANEL_H, PANEL_W, STATUS_GAP, STATUS_ROWS,
};
use crate::logo::place::Spot;

/// The panel, the warning's row, the status line, and whether the logo is
/// drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Layout {
    pub panel: Area,
    pub warn: Area,
    pub status: Area,
    pub logo: bool,
}

/// The layout for a logo at `spot` (None: no logo) on a `w` x `h` surface.
pub fn layout(spot: Option<&Spot>, w: u32, h: u32) -> Layout {
    let p = panel(w, h);
    let plain = Layout { panel: p, warn: warning(p, w), status: status(w, h), logo: false };
    let Some(s) = spot else { return plain };
    if logo_clear(s, w, h) {
        return Layout { logo: true, ..plain };
    }
    let below = u32::try_from(s.y.saturating_add(i64::from(s.h))).unwrap_or(0);
    let py = below.saturating_add(GAP);
    let sy = py.saturating_add(PANEL_H + STATUS_GAP);
    if sy.saturating_add(STATUS_ROWS + FOOT) > h {
        return plain;
    }
    let panel = (plain.panel.0, py, PANEL_W, PANEL_H);
    let warn = warning(panel, w);
    Layout { panel, warn, status: (0, sy, w, STATUS_ROWS), logo: true }
}
