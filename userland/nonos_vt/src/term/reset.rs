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

//! The full reset a program can ask for, and the settings the host makes.

use super::modes_type::Modes;
use super::screen::Screen;
use super::state::Term;
use super::tabs::default_tabs;
use super::types::CursorShape;
use crate::charset::Charsets;
use crate::color::Palette;

impl Term {
    /// RIS: both screens blank, every mode and colour back to its start. The
    /// scrollback stays; it is the user's, not the program's.
    pub(super) fn full_reset(&mut self) {
        self.primary = Screen::new(self.cols, self.rows);
        self.alt = Screen::new(self.cols, self.rows);
        self.alt_active = false;
        self.view = 0;
        self.modes = Modes::default();
        self.tabs = default_tabs(self.cols);
        self.charsets = Charsets::default();
        self.last_char = None;
        self.after_zwj = false;
        self.title.clear();
        self.title_stack.clear();
        self.title_gen = self.title_gen.wrapping_add(1);
        self.palette = self.theme.clone();
        self.cursor_shape = CursorShape::Block;
        self.touch_all();
    }

    /// Set the colours programs start from. Changes a program made are
    /// dropped: the user chose a theme.
    pub fn set_theme(&mut self, theme: Palette) {
        self.theme = theme.clone();
        self.palette = theme;
        self.touch_all();
    }

    pub fn set_cell_pixels(&mut self, w: u16, h: u16) {
        self.cell_px = (w, h);
    }
}
