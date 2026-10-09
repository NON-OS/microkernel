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

//! A cell, and the pen that writes cells.

use super::attr;
use super::pen::Pen;
use super::Underline;
use crate::color::Color;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
    pub attr: u16,
    /// OSC 8 target, 1-based into the terminal's link table; 0 is none.
    pub link: u16,
    /// Combining marks, 1-based into the line's mark table; 0 is none.
    pub mark: u16,
}

impl Cell {
    pub const BLANK: Cell =
        Cell { ch: ' ', fg: Color::Default, bg: Color::Default, attr: 0, link: 0, mark: 0 };

    /// An erased cell: blank, but keeping the pen's background, as xterm
    /// erases.
    pub fn erased(pen: &Pen) -> Cell {
        Cell { bg: pen.bg, ..Cell::BLANK }
    }

    pub fn underline(&self) -> Underline {
        Underline::from_bits((self.attr & attr::UNDERLINE_MASK) >> attr::UNDERLINE_SHIFT)
    }

    pub fn is_wide(&self) -> bool {
        self.attr & attr::WIDE != 0
    }

    pub fn is_tail(&self) -> bool {
        self.attr & attr::WIDE_TAIL != 0
    }

    /// Nothing a reader would miss if the cell were dropped from a line end.
    pub fn is_plain_blank(&self) -> bool {
        let quiet = self.attr == 0 && self.mark == 0 && self.link == 0;
        self.ch == ' ' && self.bg == Color::Default && quiet
    }
}

impl Default for Cell {
    fn default() -> Cell {
        Cell::BLANK
    }
}
