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

//! The three kernel calls the transport needs, as a trait each driver
//! implements on `nonos_libc`. Everything built on them is proved on the
//! host against a broker in memory.

/// One MMIO mapping the broker made.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MmioGrant {
    /// Where the mapping starts in the capsule's address space.
    pub user_va: u64,
    /// Bytes actually mapped. The broker stops a mapping short of an MSI-X
    /// table, so this can be less than was asked for and is always checked.
    pub length: u64,
    pub grant_id: u64,
}

/// The claimed function, as the broker lets a driver reach it.
///
/// # Safety
///
/// `mmio_map` may return `Ok` only for a mapping of `length` bytes at
/// `user_va` that stays valid for volatile reads and writes until
/// `mmio_unmap` is called with its grant id. The register windows built on
/// a grant dereference that memory without checking it again.
pub unsafe trait Broker {
    /// One 32-bit config-space read at a dword-aligned offset below 256.
    /// `None` when the broker refuses it.
    fn config_read32(&mut self, offset: u32) -> Option<u32>;

    /// Map `length` bytes of memory BAR `bar` from `offset`, both page
    /// aligned. The broker's errno when it refuses.
    fn mmio_map(&mut self, bar: u8, offset: u64, length: u64) -> Result<MmioGrant, i64>;

    /// Undo one mapping. False when the broker refused.
    fn mmio_unmap(&mut self, grant_id: u64) -> bool;
}
