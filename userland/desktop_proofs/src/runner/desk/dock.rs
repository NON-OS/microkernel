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

//! The shell's dock painted, where it is, and the pointer it follows.

use super::consts::{CLEAR, DOCK_SHADOW, PANEL, SHADOW};
use super::types::Desk;
use crate::damage::Rect as Damage;

impl Desk {
    /// The shell paints its chrome: cleared, and the dock with its shadow
    /// when its rule shows it (render/chrome.rs paint_chrome). Nothing is
    /// committed here.
    pub(super) fn paint_dock(&mut self) {
        self.chrome.fill(CLEAR);
        if !self.taskbar.visible {
            return;
        }
        let (dx, dy, dw, dh) = self.dock;
        let (x0, y0) = (dx.saturating_sub(DOCK_SHADOW), dy.saturating_sub(DOCK_SHADOW));
        let x1 = (dx + dw + DOCK_SHADOW).min(self.width);
        let y1 = (dy + dh + DOCK_SHADOW).min(self.height);
        for y in y0..y1 {
            for x in x0..x1 {
                let panel = x >= dx && x < dx + dw && y >= dy && y < dy + dh;
                self.chrome[(y * self.width + x) as usize] = if panel { PANEL } else { SHADOW };
            }
        }
    }

    /// The dock's whole area as render/layout.rs dock_area_rect has it at one
    /// to one: panel and shadow and a pixel more, down to the bottom edge.
    pub fn dock_area(&self) -> Damage {
        let (dx, dy, dw, _) = self.dock;
        let m = DOCK_SHADOW + 1;
        let (x, y) = (dx.saturating_sub(m), dy.saturating_sub(m));
        let width = (dw + 2 * m).min(self.width - x);
        Damage { x, y, width, height: self.height - y }
    }

    /// Pointer motion as the input router mirrors it to the shell, wherever
    /// it is (mirror_shell_pointer), taken by hover_reveal.
    pub fn shell_motion(&mut self, y: u32) {
        let top = self.dock_area().y;
        crate::taskbar::dock_pointer(&mut self.taskbar, y, self.height, top);
        self.shell_sync();
    }
}
