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

//! A table write made visible to a unit that reads memory without snooping
//! the CPU's caches (K12). Control.Coherent asks the unit to snoop its
//! device table fetches; the line is flushed after every write as well, so
//! a unit that ignores the bit still reads the entry just written.

use core::arch::x86_64::{_mm_clflush, _mm_mfence};

use super::error::AmdViError;
use crate::arch::x86_64::iommu::tables::sl_pte::ENTRIES;
use crate::arch::x86_64::iommu::tables::view::Frame;
use crate::memory::addr::PhysAddr;
use crate::memory::unified::phys_to_virt;

/// Write entry `index` of the table at `table`, then push its line out to
/// memory.
pub(crate) fn store(table: u64, index: usize, value: u64) -> Result<(), AmdViError> {
    Frame::at(table).map_err(|_| AmdViError::TableUnreachable)?.write(index, value)?;
    flush(table, index)
}

/// Push the cache line holding entry `index` of the table at `table` out to
/// memory, ordered before any command that follows.
pub(crate) fn flush(table: u64, index: usize) -> Result<(), AmdViError> {
    if index >= ENTRIES || table & 0xFFF != 0 {
        return Err(AmdViError::OutOfRange);
    }
    let base = phys_to_virt(PhysAddr::new(table)).ok_or(AmdViError::TableUnreachable)?;
    let at = (base.as_u64() + index as u64 * 8) as *const u8;
    // SAFETY: eK@nonos.systems - `at` is inside the directmapped table frame
    // (index checked above), mapped for the life of the kernel; CLFLUSH only
    // writes the line back and MFENCE only orders, and both are in every
    // x86_64 CPU (SSE2).
    unsafe {
        _mm_clflush(at);
        _mm_mfence();
    }
    Ok(())
}
