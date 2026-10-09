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

//! One screen: its lines, its cursor and its scroll region. The terminal has
//! two, the normal one with scrollback and the alternate one without.

use alloc::vec::Vec;

use super::save::Saved;
use crate::cell::{Cell, Pen};
use crate::line::Line;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
    pub pen: Pen,
    /// The last column was just written. The next character wraps first;
    /// anything that moves the cursor cancels it. This is why a program can
    /// fill the bottom-right cell without the screen scrolling.
    pub pending_wrap: bool,
}

pub struct Screen {
    pub lines: Vec<Line>,
    pub cur: Cursor,
    pub saved: Option<Saved>,
    /// Scroll region, inclusive rows.
    pub top: usize,
    pub bot: usize,
}

impl Screen {
    pub fn new(cols: usize, rows: usize) -> Screen {
        let lines = (0..rows).map(|_| Line::new(cols, Cell::BLANK)).collect();
        Screen { lines, cur: Cursor::default(), saved: None, top: 0, bot: rows - 1 }
    }

    pub fn rows(&self) -> usize {
        self.lines.len()
    }

    pub fn full_region(&self) -> bool {
        self.top == 0 && self.bot + 1 == self.lines.len()
    }

    pub fn reset_region(&mut self) {
        self.top = 0;
        self.bot = self.lines.len().saturating_sub(1);
    }

    pub fn blank(&self) -> Cell {
        Cell::erased(&self.cur.pen)
    }

    pub fn line(&mut self) -> &mut Line {
        let y = self.cur.y.min(self.lines.len() - 1);
        &mut self.lines[y]
    }
}
