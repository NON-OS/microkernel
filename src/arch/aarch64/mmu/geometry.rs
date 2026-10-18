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

//! The shape of the kernel's tables: the 4 KiB granule's 512-entry tables,
//! where a level 2 entry maps 2 MiB and a level 1 entry 1 GiB (Arm ARM, DDI
//! 0487, the VMSAv8-64 translation table format).

pub(super) const BLOCK_2M: u64 = 2 * 1024 * 1024;
/// Entries in a table, and so the span of one level 1 entry: 512 blocks.
pub(super) const ENTRIES: usize = 512;
pub(super) const GIB: u64 = ENTRIES as u64 * BLOCK_2M;

/// Base of the kernel's direct map, matching what shared code adds to a
/// physical address to get a pointer.
pub const KERNEL_SPACE_START: u64 = 0xFFFF_8000_0000_0000;
/// Level 0 entry the direct map hangs off, bits 47:39 of its base.
pub(super) const DIRECT_L0_INDEX: usize = 256;
