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

use core::mem::size_of;

use super::super::super::types::map_window::map_window;
use super::super::super::types::{BootHandoffV1, MemoryMapEntry};
use super::super::error::HandoffError;
use crate::memory::layout::{DIRECTMAP_BASE, DIRECTMAP_SIZE};

pub(super) fn check(handoff: &BootHandoffV1) -> Result<(), HandoffError> {
    if handoff.mmap.ptr == 0 {
        return Ok(());
    }
    let expected = size_of::<MemoryMapEntry>() as u32;
    let m = handoff.mmap;
    if m.entry_size != expected {
        return Err(HandoffError::MemoryMapEntrySize { expected, got: m.entry_size });
    }
    // A count the buffer cannot hold, or a buffer the direct map does not
    // reach, is refused here rather than read as no memory later.
    if map_window(m.ptr, m.entry_size, m.entry_count, DIRECTMAP_BASE, DIRECTMAP_SIZE).is_none() {
        return Err(HandoffError::InvalidData);
    }
    Ok(())
}
