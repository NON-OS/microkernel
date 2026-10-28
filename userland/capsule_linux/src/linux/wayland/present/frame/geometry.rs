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

//! Where a guest's content sits inside its frame, and how large the frame
//! is. Pure, so the host proofs hold it.

use nonos_toolkit::decorations::content_rect_at;

/// A size the toolkit's frame is measured at: its margins do not depend on
/// the window's size, only on the display's scale.
const PROBE: u32 = 1024;

/// A window `w` by `h` with the content at (`cx`, `cy`) inside it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Around {
    pub w: u32,
    pub h: u32,
    pub cx: u32,
    pub cy: u32,
    /// Drawn as a maximised window's frame: no margin round it.
    pub max: bool,
}

impl Around {
    /// No frame: the window is the content.
    pub const fn bare(w: u32, h: u32) -> Around {
        Around { w, h, cx: 0, cy: 0, max: false }
    }

    pub fn framed(&self) -> bool {
        self.cx != 0 || self.cy != 0
    }

    /// Whether (`x`, `y`) on the window is on the content, and where on it.
    pub fn on_content(&self, x: i32, y: i32, cw: u32, ch: u32) -> Option<(i32, i32)> {
        let (x, y) = (x - self.cx as i32, y - self.cy as i32);
        (x >= 0 && y >= 0 && (x as u32) < cw && (y as u32) < ch).then_some((x, y))
    }
}

/// The frame around content `w` by `h` at `quarters` of display scale: none
/// when `fills` (full screen, or a guest that draws its own), and without
/// the toolkit's margin round it when `max`.
pub fn around_at(w: u32, h: u32, quarters: u32, fills: bool, max: bool) -> Around {
    if fills {
        return Around::bare(w, h);
    }
    let c = content_rect_at(PROBE, PROBE, max, quarters);
    Around { w: w + (PROBE - c.w), h: h + (PROBE - c.h), cx: c.x, cy: c.y, max }
}
