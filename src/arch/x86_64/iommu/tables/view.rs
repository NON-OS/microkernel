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
//! A frame a remapping unit reads or writes behind the compiler's back: a
//! table it walks, a ring it fetches from, a word it stores a status into.

use crate::memory::addr::PhysAddr;
use crate::memory::unified::phys_to_virt;

use super::super::types::VtdError;
use super::out_of_frame::OutOfFrame;
use super::sl_pte::ENTRIES;

/// One 4 KiB frame as 512 quadwords, reached through the directmap. Not a
/// slice: a `&mut [u64]` over memory the hardware also touches lets the
/// compiler keep, merge or reorder the stores, and a unit can then read half
/// an entry. Every access here is volatile and at a checked index.
#[derive(Clone, Copy)]
pub struct Frame {
    base: *mut u64,
}

impl Frame {
    pub fn at(phys: u64) -> Result<Self, VtdError> {
        if phys & 0xFFF != 0 {
            return Err(VtdError::AddressMisaligned);
        }
        let virt = phys_to_virt(PhysAddr::new(phys)).ok_or(VtdError::TableUnreachable)?;
        Ok(Self { base: virt.as_u64() as *mut u64 })
    }

    pub fn read(self, index: usize) -> Result<u64, OutOfFrame> {
        if index >= ENTRIES {
            return Err(OutOfFrame);
        }
        // SAFETY: eK@nonos.systems - `base` is the directmap view of a page
        // aligned frame this module owns, mapped for the life of the kernel,
        // and `index` is inside its 512 quadwords.
        Ok(unsafe { self.base.add(index).read_volatile() })
    }

    pub fn write(self, index: usize, value: u64) -> Result<(), OutOfFrame> {
        if index >= ENTRIES {
            return Err(OutOfFrame);
        }
        // SAFETY: eK@nonos.systems - as in `read`.
        unsafe { self.base.add(index).write_volatile(value) };
        Ok(())
    }

    /// The state a fresh table must have before anything points at it.
    pub fn zero(self) {
        for index in 0..ENTRIES {
            // SAFETY: eK@nonos.systems - as in `read`; `index` < ENTRIES.
            unsafe { self.base.add(index).write_volatile(0) };
        }
    }
}
