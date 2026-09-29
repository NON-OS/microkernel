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

//! Killing back to a word or line start, and Enter, in the line a program
//! has not been sent yet.

use super::cooked::{erase, Cooked, Effect};

impl Cooked {
    pub fn kill_line(&mut self, fx: &mut Effect) {
        while let Some(c) = self.line.pop() {
            erase(&mut fx.echo, c);
        }
    }

    pub fn kill_word(&mut self, fx: &mut Effect) {
        while self.line.last().is_some_and(|c| c.is_whitespace()) {
            self.backspace(fx);
        }
        while self.line.last().is_some_and(|c| !c.is_whitespace()) {
            self.backspace(fx);
        }
    }

    pub fn enter(&mut self, fx: &mut Effect) {
        fx.echo.extend_from_slice(b"\n");
        for c in self.line.drain(..) {
            let mut buf = [0u8; 4];
            fx.send.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
        }
        fx.send.push(b'\n');
    }
}
