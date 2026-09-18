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
//! File bodies are borrowed from the image so a hundred megabytes are never
//! copied; tables, directories and tails are built once and owned.

use alloc::vec::Vec;

pub enum Source<'a> {
    Owned(Vec<u8>),
    Borrowed(&'a [u8]),
}

pub struct Job<'a> {
    pub lba: u64,
    pub source: Source<'a>,
}

impl<'a> Job<'a> {
    pub fn owned(lba: u64, bytes: Vec<u8>) -> Job<'a> {
        Job { lba, source: Source::Owned(bytes) }
    }

    pub fn borrowed(lba: u64, bytes: &'a [u8]) -> Job<'a> {
        Job { lba, source: Source::Borrowed(bytes) }
    }

    pub fn bytes(&self) -> &[u8] {
        match &self.source {
            Source::Owned(v) => v,
            Source::Borrowed(b) => b,
        }
    }

    pub fn len(&self) -> usize {
        self.bytes().len()
    }
}
