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

//! The FIFO as 32-bit words in the mapped BAR2. The device reads commands
//! from STOP to NEXT_CMD; the driver writes behind NEXT_CMD and moves it.

use core::ptr::{read_volatile, write_volatile};

pub struct Fifo {
    va: u64,
    words: usize,
    pub min: u32,
    pub max: u32,
}

impl Fifo {
    pub const fn new(va: u64, bytes: u64) -> Self {
        Self { va, words: (bytes / 4) as usize, min: 0, max: 0 }
    }

    pub fn bytes(&self) -> u32 {
        (self.words * 4) as u32
    }

    fn at(&self, word: usize) -> *mut u32 {
        assert!(word < self.words, "FIFO word {word} past the mapping");
        (self.va as usize + word * 4) as *mut u32
    }

    pub fn get(&self, word: usize) -> u32 {
        // SAFETY: `at` holds the word inside the broker-mapped FIFO.
        unsafe { read_volatile(self.at(word)) }
    }

    pub fn set(&self, word: usize, value: u32) {
        // SAFETY: as for `get`.
        unsafe { write_volatile(self.at(word), value) }
    }
}
