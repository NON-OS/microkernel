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

//! Handing a destroyed AMD-Vi domain's tables back to the frame allocator,
//! on the walk VT-d uses.

use super::super::error::AmdViError;
use super::super::pte::LEVELS;
use super::super::pte_child::child_table;
use crate::arch::x86_64::iommu::tables::release_walk::release_walk;
use crate::arch::x86_64::iommu::tables::view::Frame;
use crate::memory::addr::PhysAddr;
use crate::memory::frame_alloc::manager::deallocate_frame;

/// Only after the domain's devices are detached and its cached translations
/// flushed, the completion wait passed.
pub(super) fn release_tables(root: u64) -> Result<usize, AmdViError> {
    let read = |table: u64, index: usize| -> Result<u64, AmdViError> {
        Ok(Frame::at(table).map_err(|_| AmdViError::TableUnreachable)?.read(index)?)
    };
    let mut free = |table: u64| -> Result<(), AmdViError> {
        deallocate_frame(PhysAddr::new(table)).map_err(|_| AmdViError::TableNotFreed)
    };
    release_walk(root, LEVELS, &|e, l| child_table(e, l), &read, &mut free)
}
