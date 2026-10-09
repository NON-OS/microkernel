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

use super::super::constants::ALLOCATION_MAGIC;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AllocationHeader {
    pub magic: u32,
    pub size: usize,
    pub canary_offset: usize,
    pub allocated_at: u64,
}

impl AllocationHeader {
    pub const fn new(size: usize, timestamp: u64) -> Self {
        Self { magic: ALLOCATION_MAGIC, size, canary_offset: size, allocated_at: timestamp }
    }

    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.magic == ALLOCATION_MAGIC
    }
}

/// Where the data starts in a block aligned to `align`: past the header,
/// rounded up so the data itself has the caller's alignment. The header sits
/// directly before the data either way, where free and verify look for it.
pub(crate) const fn data_offset(align: usize) -> usize {
    let header = core::mem::size_of::<AllocationHeader>();
    (header + align - 1) & !(align - 1)
}
