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

//! The host register block, as the shipping `hw/mmio.rs` lays it out, with
//! each 32-bit access answered by the modelled reader in order, so a test
//! sees every write the driver makes and the reader reacts to it at once.

use nonos_libc::MMIO_VA;

#[derive(Clone, Copy)]
pub struct Regs {
    base: u64,
}

impl Regs {
    pub const fn new(base: u64) -> Self {
        Self { base }
    }

    /// An access through any other base reaches nothing, as an unmapped
    /// address would not; the model reads all ones for it.
    pub fn r32(&self, offset: u32) -> u32 {
        if self.base != MMIO_VA {
            return u32::MAX;
        }
        crate::model::with(|m| m.r32(offset))
    }

    pub fn w32(&self, offset: u32, value: u32) {
        if self.base == MMIO_VA {
            crate::model::with(|m| m.w32(offset, value));
        }
    }
}
