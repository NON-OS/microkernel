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

//! Terminal input gathered into lines. A line is held to `MAX_LINE` bytes:
//! input past that is dropped up to the next newline and reported once, so a
//! peer that never sends a newline cannot make the capsule allocate without
//! end.

use alloc::vec::Vec;

/// The longest command line the shell holds.
pub const MAX_LINE: usize = 4096;

#[derive(Debug, PartialEq, Eq)]
pub enum Line {
    /// A whole line, without its newline.
    Command(Vec<u8>),
    /// A line over `MAX_LINE` was dropped.
    TooLong,
}

#[derive(Default)]
pub struct Lines {
    held: Vec<u8>,
    dropping: bool,
}

impl Lines {
    /// Take `bytes` and return every line they complete, in order.
    pub fn feed(&mut self, bytes: &[u8]) -> Vec<Line> {
        let mut out = Vec::new();
        for part in bytes.split_inclusive(|b| *b == b'\n') {
            let (text, ends) = match part.split_last() {
                Some((b'\n', text)) => (text, true),
                _ => (part, false),
            };
            self.take(text, &mut out);
            if ends {
                self.end(&mut out);
            }
        }
        out
    }

    fn take(&mut self, text: &[u8], out: &mut Vec<Line>) {
        if self.dropping {
            return;
        }
        if self.held.len().saturating_add(text.len()) > MAX_LINE {
            self.held = Vec::new();
            self.dropping = true;
            out.push(Line::TooLong);
            return;
        }
        self.held.extend_from_slice(text);
    }

    fn end(&mut self, out: &mut Vec<Line>) {
        if !core::mem::take(&mut self.dropping) {
            out.push(Line::Command(core::mem::take(&mut self.held)));
        }
    }
}
