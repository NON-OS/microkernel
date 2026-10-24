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

//! A grant's frames, taken and zeroed, and given back at once when they run
//! into memory firmware keeps for itself. RAM the allocator holds is never
//! firmware's; if the memory map and the namespace disagree, the device gets
//! neither.

use super::alloc::{alloc_and_zero, free};
use crate::hardware::broker::dma::types::DmaMapError;
use crate::hardware::broker::reserved;

/// The stage that refused a run, as `fail` names it, and its error.
type Refused = (&'static str, DmaMapError);

/// The run's start, or why there is none. `uncached`: from the uncached
/// pool or not at all (`uncached.rs`).
pub(super) fn take(pages: u64, length: u64, flags: u32, uncached: bool) -> Result<u64, Refused> {
    let phys_start = alloc_and_zero(pages, length, flags, uncached).map_err(|e| ("alloc", e))?;
    if reserved::refuses(b"DMA", reserved::MEMORY, phys_start, length, false) {
        free(phys_start, pages);
        return Err(("reserved", DmaMapError::NoMemory));
    }
    Ok(phys_start)
}
