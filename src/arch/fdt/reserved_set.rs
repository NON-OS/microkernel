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

/// Reserved ranges kept: the blob, the initrd, and firmware's own.
const MAX_RESERVED: usize = 16;

/// Physical ranges the allocator must never hand out: the device tree blob,
/// the initrd, and what firmware keeps through the reservation block and
/// `/reserved-memory`. Fixed storage, filled before the heap exists.
#[derive(Debug, Clone, Copy)]
pub struct Reserved {
    ranges: [(u64, u64); MAX_RESERVED],
    count: usize,
}

impl Reserved {
    pub const EMPTY: Self = Self { ranges: [(0, 0); MAX_RESERVED], count: 0 };

    /// Keep `[base, base + size)`. Full is an error, not a drop: a range
    /// left out is memory handed out while firmware still owns it.
    pub fn add(&mut self, base: u64, size: u64) -> Result<(), &'static [u8]> {
        if size == 0 {
            return Ok(());
        }
        let end = base.checked_add(size).ok_or(&b"a reserved range wraps the address space"[..])?;
        let slot = self.ranges.get_mut(self.count).ok_or(&b"more reserved ranges than kept"[..])?;
        *slot = (base, end);
        self.count += 1;
        Ok(())
    }

    /// The ranges as `[start, end)`.
    pub fn ranges(&self) -> &[(u64, u64)] {
        &self.ranges[..self.count]
    }

    /// Whether any range meets `[start, end)`.
    pub fn overlaps(&self, start: u64, end: u64) -> bool {
        self.ranges().iter().any(|&(a, b)| a < end && start < b)
    }
}
