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

//! Literals and matches into the output window.

use super::copy_words::copy_match;
use super::out::Out;
use super::types::End;

impl Out {
    #[inline(always)]
    pub fn lit(&mut self, b: u8) -> Result<(), End> {
        if !self.roomy() && self.pos >= self.buf.len() {
            return Err(End::Capped);
        }
        self.buf[self.pos] = b;
        self.pos += 1;
        Ok(())
    }

    /// Appends `len` bytes starting `dist` back. The source may overlap
    /// the bytes being written, which repeats its first `dist` bytes.
    #[inline(always)]
    pub fn copy(&mut self, dist: usize, len: usize) -> Result<(), End> {
        if dist == 0 || dist > self.pos {
            return Err(End::Corrupt);
        }
        let (s, d) = (self.pos - dist, self.pos);
        if !self.roomy() {
            let n = len.min(self.buf.len() - d);
            for i in 0..n {
                self.buf[d + i] = self.buf[s + i];
            }
            self.pos += n;
            return if n < len { Err(End::Capped) } else { Ok(()) };
        }
        copy_match(&mut self.buf, d, dist, len);
        self.pos += len;
        Ok(())
    }
}
