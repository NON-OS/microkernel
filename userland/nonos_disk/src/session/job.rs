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

//! One contiguous write: where it starts and where its bytes come from.
//! File bodies are borrowed from the image; everything built is shared, so
//! the read-back compares against the bytes that were sent and each copy of
//! the FAT or of the entry array is the same allocation.

use alloc::rc::Rc;
use alloc::vec::Vec;
use core::fmt;
use core::ops::Range;

#[derive(Clone)]
enum Source<'a> {
    Borrowed(&'a [u8]),
    Shared(Rc<Vec<u8>>, Range<usize>),
}

#[derive(Clone)]
pub struct Job<'a> {
    pub lba: u64,
    source: Source<'a>,
}

impl<'a> Job<'a> {
    pub fn borrowed(lba: u64, bytes: &'a [u8]) -> Job<'a> {
        Job { lba, source: Source::Borrowed(bytes) }
    }

    pub fn owned(lba: u64, bytes: Vec<u8>) -> Job<'a> {
        Self::shared(lba, Rc::new(bytes))
    }

    pub fn shared(lba: u64, bytes: Rc<Vec<u8>>) -> Job<'a> {
        let all = 0..bytes.len();
        Self::part(lba, bytes, all)
    }

    /// `range` of `bytes`, which the caller keeps to whole sectors.
    pub fn part(lba: u64, bytes: Rc<Vec<u8>>, range: Range<usize>) -> Job<'a> {
        Job { lba, source: Source::Shared(bytes, range) }
    }

    pub fn bytes(&self) -> &[u8] {
        match &self.source {
            Source::Borrowed(b) => b,
            Source::Shared(v, r) => &v[r.clone()],
        }
    }

    pub fn len(&self) -> usize {
        self.bytes().len()
    }
}

/// Where and how much, never the bytes themselves.
impl fmt::Debug for Job<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Job {{ lba: {}, bytes: {} }}", self.lba, self.len())
    }
}
