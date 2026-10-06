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

//! The line a foreground program has not been sent yet, edited as a tty's
//! canonical mode edits it. Typed characters echo, Backspace erases the
//! last one from the screen as well as the line, Ctrl+U the whole line and
//! Ctrl+W the last word, and Enter sends the line with its newline.

use alloc::vec::Vec;

use nonos_vt::width::width;

#[derive(Default)]
pub struct Cooked {
    pub line: Vec<char>,
}

/// What an edit asks the terminal to do.
#[derive(Default)]
pub struct Effect {
    /// Bytes to put on the screen.
    pub echo: Vec<u8>,
    /// Bytes to send to the program.
    pub send: Vec<u8>,
}

pub(super) fn erase(echo: &mut Vec<u8>, c: char) {
    for _ in 0..width(c).max(1) {
        echo.extend_from_slice(b"\x08 \x08");
    }
}

impl Cooked {
    pub fn char(&mut self, c: char, fx: &mut Effect) {
        self.line.push(c);
        let mut buf = [0u8; 4];
        fx.echo.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
    }

    pub fn backspace(&mut self, fx: &mut Effect) {
        if let Some(c) = self.line.pop() {
            erase(&mut fx.echo, c);
        }
    }
}
