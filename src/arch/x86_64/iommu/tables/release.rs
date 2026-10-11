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

//! Handing a destroyed domain's second-level tables back to the frame
//! allocator, which zeroes each frame as it takes it.

use super::release_walk::{child_table, release_walk};
use super::view::Frame;
use crate::arch::x86_64::iommu::types::VtdError;
use crate::memory::addr::PhysAddr;
use crate::memory::frame_alloc::manager::deallocate_frame;

/// Only once no unit can walk them: the domain's devices denied and every
/// unit's caches invalidated, the wait descriptor completed.
pub fn release_tables(root: u64, levels: u8) -> Result<usize, VtdError> {
    let read =
        |table: u64, index: usize| -> Result<u64, VtdError> { Ok(Frame::at(table)?.read(index)?) };
    let mut free = |table: u64| -> Result<(), VtdError> {
        deallocate_frame(PhysAddr::new(table)).map_err(|_| VtdError::TableNotFreed)
    };
    release_walk(root, levels, &|e, l| child_table(e, l), &read, &mut free)
}
