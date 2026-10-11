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

//! A guest's memory for the proofs: one mapped span at `base`, and a fault
//! for any byte outside it, which is what a peer read or write does with a
//! pointer the guest does not hold. Every read and write is counted.

use core::cell::{Cell, RefCell};

use crate::linux::guest::memory::Memory;

pub struct Fake {
    pub base: u64,
    pub bytes: RefCell<Vec<u8>>,
    pub writes: Cell<u32>,
}

impl Fake {
    pub fn new(base: u64, len: usize) -> Fake {
        Fake { base, bytes: RefCell::new(vec![0; len]), writes: Cell::new(0) }
    }

    /// Nothing at all is always there, as a peer copy of no bytes succeeds.
    fn span(&self, at: u64, len: usize) -> Option<core::ops::Range<usize>> {
        if len == 0 {
            return Some(0..0);
        }
        let from = usize::try_from(at.checked_sub(self.base)?).ok()?;
        let to = from.checked_add(len)?;
        (to <= self.bytes.borrow().len()).then_some(from..to)
    }

    pub fn put(&self, at: u64, bytes: &[u8]) {
        let Some(r) = self.span(at, bytes.len()) else {
            panic!("put outside the fake: {at:#x}");
        };
        self.bytes.borrow_mut()[r].copy_from_slice(bytes);
    }

    pub fn get(&self, at: u64, len: usize) -> Vec<u8> {
        self.read_at(at, len).unwrap_or_default()
    }
}

impl Memory for Fake {
    fn read_at(&self, at: u64, len: usize) -> Option<Vec<u8>> {
        let r = self.span(at, len)?;
        Some(self.bytes.borrow()[r].to_vec())
    }

    fn write_at(&self, at: u64, bytes: &[u8]) -> i64 {
        let Some(r) = self.span(at, bytes.len()) else {
            return -14;
        };
        self.writes.set(self.writes.get() + 1);
        self.bytes.borrow_mut()[r].copy_from_slice(bytes);
        bytes.len() as i64
    }
}
