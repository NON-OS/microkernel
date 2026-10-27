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

//! One shared file map as the family's table keeps it, and its two ranges:
//! where it sits in its process, and which bytes of its file it shows.
//! Pure, so the host proofs hold it.

use alloc::vec::Vec;

/// A file one process of the family mapped MAP_SHARED.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    /// The process, as a peer call names it.
    pub pid: u32,
    /// Where the map starts in that process and how long it is, in pages.
    pub at: u64,
    pub span: u64,
    /// The file as its descriptor named it, and the offset `at` shows.
    pub path: Vec<u8>,
    pub off: u64,
    /// Mapped writable at some time, so written back from then on.
    pub writer: bool,
    /// Its descriptor was open to write, so it may become a writer.
    pub may_write: bool,
    /// Its file was unlinked or replaced: no file of that name is its.
    pub orphan: bool,
}

/// What two ranges share, or None when they meet nowhere.
pub fn overlap(a: (u64, u64), b: (u64, u64)) -> Option<(u64, u64)> {
    let (lo, hi) = (a.0.max(b.0), a.1.min(b.1));
    (lo < hi).then_some((lo, hi))
}

/// A range of a file cut at its end.
pub fn clip(r: (u64, u64), end: u64) -> Option<(u64, u64)> {
    overlap(r, (0, end))
}

impl Entry {
    /// The bytes of the file it shows.
    pub fn file(&self) -> (u64, u64) {
        (self.off, self.off.saturating_add(self.span))
    }

    /// Where it sits in its process.
    pub fn place(&self) -> (u64, u64) {
        (self.at, self.at.saturating_add(self.span))
    }

    /// The address that shows the file's byte `at`, one `file()` holds.
    pub fn addr(&self, at: u64) -> u64 {
        self.at.saturating_add(at - self.off)
    }

    /// Whether it shows the file now at `path`.
    pub fn shows(&self, path: &[u8]) -> bool {
        !self.orphan && self.path == path
    }
}
