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

//! The output window: bytes `..pos` of `buf` are the output so far. `buf`
//! is never longer than `cap`; while it has `SLACK` bytes past `pos`, any
//! literal or match fits, and a match may copy in whole 16-byte words.

use alloc::vec;
use alloc::vec::Vec;

/// The longest match plus one word of overshoot.
pub const SLACK: usize = 258 + 16;

pub struct Out {
    pub buf: Vec<u8>,
    pub pos: usize,
    pub cap: usize,
}

impl Out {
    /// An empty output with room for about `hint` bytes.
    pub fn new(cap: usize, hint: usize) -> Self {
        Out { buf: vec![0; hint.saturating_add(SLACK).min(cap)], pos: 0, cap }
    }

    /// True when a literal or a whole match fits without further checks.
    #[inline(always)]
    pub fn roomy(&mut self) -> bool {
        self.pos + SLACK <= self.buf.len() || self.grow()
    }

    #[cold]
    fn grow(&mut self) -> bool {
        if self.buf.len() >= self.cap {
            return false;
        }
        let want = self.buf.len().saturating_mul(2).max(self.pos + SLACK + 4096);
        self.buf.resize(want.min(self.cap), 0);
        self.pos + SLACK <= self.buf.len()
    }

    /// Appends as much of `data` as the cap allows; returns how much.
    pub fn extend(&mut self, data: &[u8]) -> usize {
        let n = data.len().min(self.cap - self.pos);
        if self.pos + n > self.buf.len() {
            let want = self.buf.len().saturating_mul(2).max(self.pos + n + SLACK);
            self.buf.resize(want.min(self.cap), 0);
        }
        self.buf[self.pos..self.pos + n].copy_from_slice(&data[..n]);
        self.pos += n;
        n
    }

    pub fn finish(mut self) -> Vec<u8> {
        self.buf.truncate(self.pos);
        self.buf
    }
}
