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

//! The displays NONOS is laid out for, and every canvas setup and the
//! installer can be handed on them.

use crate::canvas_scale::scale_for;

/// Laptop and desktop panels from the smallest to 4K.
pub const DISPLAYS: [(u32, u32); 6] =
    [(1366, 768), (1440, 900), (1920, 1080), (2560, 1440), (2560, 1600), (3840, 2160)];

/// A canvas a client draws on: its size, the display it is shown on, and the
/// compositor's scale between the two.
#[derive(Clone, Copy, Debug)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub display: (u32, u32),
    pub compositor: u32,
}

/// Every canvas: each display's as the compositor makes it, and for a dense
/// display also the full-size one it falls back to when it cannot map the
/// half-size canvas.
pub fn canvases() -> Vec<Canvas> {
    let mut out = Vec::new();
    for (w, h) in DISPLAYS {
        let s = scale_for(w, h);
        out.push(Canvas { width: w / s, height: h / s, display: (w, h), compositor: s });
        if s > 1 {
            out.push(Canvas { width: w, height: h, display: (w, h), compositor: 1 });
        }
    }
    out
}
