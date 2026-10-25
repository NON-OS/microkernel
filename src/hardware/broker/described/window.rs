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

/// Bus addresses `[bus_base, bus_base + size)` reach CPU physical memory at
/// `[cpu_base, cpu_base + size)`: what the device must be told for a frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BusWindow {
    pub bus_base: u64,
    pub cpu_base: u64,
    pub size: u64,
}

const FOUR_GIB: u64 = 1 << 32;

impl BusWindow {
    /// The bus address of `[phys, phys + len)`, when all of it is inside.
    pub fn bus_address(&self, phys: u64, len: u64) -> Option<u64> {
        let offset = phys.checked_sub(self.cpu_base)?;
        if offset > self.size || len > self.size - offset {
            return None;
        }
        self.bus_base.checked_add(offset)
    }

    /// Whether the window ends at or below 4 GiB of CPU memory, so frames
    /// for it come from where the low DMA pool is.
    pub fn ends_below_4g(&self) -> bool {
        self.cpu_base.checked_add(self.size).is_some_and(|end| end <= FOUR_GIB)
    }
}
