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

use nonos_vt::Palette;

use super::types::Scrollback;
use crate::term::theme::types::Theme;

impl Scrollback {
    /// Match the screen to the window. A resize re-wraps what is there.
    /// True when the size in cells changed.
    pub fn fit(&mut self, cols: usize, rows: usize, cell_w: u32, cell_h: u32) -> bool {
        let changed = (cols, rows) != (self.vt.cols(), self.vt.rows());
        self.vt.resize(cols, rows);
        self.vt.set_cell_pixels(
            cell_w.min(u16::MAX as u32) as u16,
            cell_h.min(u16::MAX as u32) as u16,
        );
        changed
    }

    /// Program colours start from the window's theme. Only a change of
    /// theme resets them, so a colour a program set survives a repaint.
    pub fn follow_theme(&mut self, index: u16, t: &Theme) {
        if self.theme_of == Some(index) {
            return;
        }
        let mut p = Palette::xterm();
        p.colors[..16].copy_from_slice(crate::term::theme::ansi::base16_for(t.bg));
        p.fg = t.fg & 0x00FF_FFFF;
        p.bg = t.bg & 0x00FF_FFFF;
        p.cursor = t.accent & 0x00FF_FFFF;
        self.vt.set_theme(p);
        self.theme_of = Some(index);
    }
}
