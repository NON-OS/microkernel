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

//! mmap's flags: shared or private, and an exact address or a hint.

pub const MAP_SHARED: u64 = 0x01;
pub const MAP_PRIVATE: u64 = 0x02;

/// MAP_SHARED with every flag checked: shared, as far as this capsule goes.
pub(super) const MAP_SHARED_VALIDATE: u64 = 0x03;

/// The bits that say shared or private.
pub(super) const MAP_TYPE: u64 = 0x0f;
pub const MAP_FIXED: u64 = 0x10;
pub const MAP_FIXED_NOREPLACE: u64 = 0x10_0000;

/// Linux's default vm.mmap_min_addr. Nothing is mapped at an exact address
/// below it, so a null pointer, or one a little past null, is never memory.
pub const MIN_ADDR: u64 = 0x1_0000;

/// True for MAP_FIXED and MAP_FIXED_NOREPLACE: the exact address or failure.
pub fn exact(flags: u64) -> bool {
    flags & (MAP_FIXED | MAP_FIXED_NOREPLACE) != 0
}
