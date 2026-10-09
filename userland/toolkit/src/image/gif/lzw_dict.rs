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

use alloc::vec;
use alloc::vec::Vec;

pub(super) const MAX_CODES: usize = 4096;

/* The LZW string table: each code past the roots is a prefix code plus one
 * suffix byte. `free` is the next code to assign. */
pub(super) struct Dict {
    pub prefix: Vec<u16>,
    pub suffix: Vec<u8>,
    pub clear: u16,
    pub free: u16,
}

impl Dict {
    pub fn new(clear: u16) -> Self {
        let mut suffix = vec![0u8; MAX_CODES];
        for (i, s) in suffix.iter_mut().enumerate().take(clear as usize) {
            *s = i as u8;
        }
        Self { prefix: vec![0u16; MAX_CODES], suffix, clear, free: clear + 2 }
    }

    pub fn root(&self, code: u16) -> u8 {
        self.suffix[code as usize]
    }

    /* Add `old` + `first` as the next code while the table has room. */
    pub fn add(&mut self, old: u16, first: u8) {
        if (self.free as usize) < MAX_CODES {
            self.prefix[self.free as usize] = old;
            self.suffix[self.free as usize] = first;
            self.free += 1;
        }
    }
}
