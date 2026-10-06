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

//! Bytes from the program in, through the parser, onto the screen.

use super::screen::Screen;
use super::state::Term;

impl Term {
    pub fn feed(&mut self, bytes: &[u8]) {
        /*
         * The parser is held outside the terminal while it runs, so it can
         * hand the terminal to itself as the handler.
         */
        let mut parser = core::mem::take(&mut self.parser);
        parser.feed(self, bytes);
        self.parser = parser;
    }

    pub(super) fn scr(&mut self) -> &mut Screen {
        if self.alt_active {
            &mut self.alt
        } else {
            &mut self.primary
        }
    }

    pub(super) fn scr_ref(&self) -> &Screen {
        if self.alt_active {
            &self.alt
        } else {
            &self.primary
        }
    }

    pub(super) fn touch(&mut self, y: usize) {
        if let Some(d) = self.dirty.get_mut(y) {
            *d = true;
        }
    }

    pub(super) fn touch_all(&mut self) {
        for d in self.dirty.iter_mut() {
            *d = true;
        }
    }
}
