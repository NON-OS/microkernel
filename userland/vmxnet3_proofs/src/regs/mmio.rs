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

//! A register page the driver's `Regs` reaches, answered by this thread's
//! device. The base names the page: 0 is BAR0, 1 is BAR1.

use crate::model::{with_device, Bar};

#[derive(Clone, Copy)]
pub struct Mmio {
    bar: Bar,
}

impl Mmio {
    pub const fn new(base: u64) -> Self {
        Self { bar: if base == 0 { Bar::Pt } else { Bar::Vd } }
    }

    /// # Safety
    /// None needed on the host; kept unsafe to match the shipping signature.
    pub unsafe fn r32(&self, offset: usize) -> u32 {
        with_device(|d| d.read32(self.bar, offset))
    }

    /// # Safety
    /// As for `r32`.
    pub unsafe fn w32(&self, offset: usize, value: u32) {
        with_device(|d| d.write32(self.bar, offset, value));
    }
}
