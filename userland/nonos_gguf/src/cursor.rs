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

//! The header walked front to back through a 4 KiB window, so a vocabulary's
//! short strings cost a few hundred reads. Nothing past the file's length.

use crate::error::GgufError;
use crate::source::ReadAt;

const WINDOW: usize = 4096;

pub(crate) struct Cursor<'a, S: ReadAt> {
    src: &'a mut S,
    pub pos: u64,
    pub len: u64,
    win: [u8; WINDOW],
    win_at: u64,
    win_len: usize,
}

impl<'a, S: ReadAt> Cursor<'a, S> {
    pub(crate) fn new(src: &'a mut S, len: u64) -> Self {
        Cursor { src, pos: 0, len, win: [0; WINDOW], win_at: 0, win_len: 0 }
    }

    /// The next `N` bytes, or Truncated if the file ends first.
    pub(crate) fn take<const N: usize>(&mut self) -> Result<[u8; N], GgufError> {
        let mut out = [0u8; N];
        let end = self.pos.checked_add(N as u64).ok_or(GgufError::Truncated { at: self.pos })?;
        if end > self.len {
            return Err(GgufError::Truncated { at: self.pos });
        }
        let inside = self.pos >= self.win_at && end <= self.win_at + self.win_len as u64;
        if !inside {
            self.refill()?;
        }
        let from = (self.pos - self.win_at) as usize;
        out.copy_from_slice(&self.win[from..from + N]);
        self.pos = end;
        Ok(out)
    }

    /// Step over `n` bytes, or Truncated if the file ends first.
    pub(crate) fn skip(&mut self, n: u64) -> Result<(), GgufError> {
        let end = self.pos.checked_add(n).ok_or(GgufError::Truncated { at: self.pos })?;
        if end > self.len {
            return Err(GgufError::Truncated { at: self.pos });
        }
        self.pos = end;
        Ok(())
    }

    fn refill(&mut self) -> Result<(), GgufError> {
        let n = (WINDOW as u64).min(self.len - self.pos) as usize;
        if !self.src.read_at(self.pos, &mut self.win[..n]) {
            return Err(GgufError::Source { at: self.pos });
        }
        self.win_at = self.pos;
        self.win_len = n;
        Ok(())
    }
}
