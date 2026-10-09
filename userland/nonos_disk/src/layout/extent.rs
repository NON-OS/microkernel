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

//! A run of whole sectors.

use crate::sink::SECTOR_SIZE;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Extent {
    pub first: u64,
    pub sectors: u64,
}

impl Extent {
    pub const fn new(first: u64, sectors: u64) -> Extent {
        Extent { first, sectors }
    }

    /// The first sector past it.
    pub fn end(&self) -> u64 {
        self.first + self.sectors
    }

    /// Its last sector, as a GPT entry records it.
    pub fn last(&self) -> u64 {
        self.end() - 1
    }

    pub fn contains(&self, lba: u64) -> bool {
        lba >= self.first && lba < self.end()
    }

    pub fn overlaps(&self, other: &Extent) -> bool {
        self.first < other.end() && other.first < self.end()
    }

    pub fn bytes(&self) -> u64 {
        self.sectors.saturating_mul(SECTOR_SIZE as u64)
    }
}
