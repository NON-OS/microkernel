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

use super::super::flags;
use crate::arch::aarch64::mmu::MemoryType;

/// Registers are Device-nGnRnE: no access gathered, reordered or acknowledged
/// early. Anything else uncached is RAM, so Normal Non-cacheable, which takes
/// unaligned accesses and DC ZVA as RAM must; Device memory faults on both,
/// and a DMA ring a compiler `memset`s or copies into would trap. A
/// write-through request is the x86 write-combining one, and Normal
/// Non-cacheable gathers writes as write-combining does (Arm ARM, DDI 0487,
/// the memory types).
pub(super) const fn memory_type(flags: u64) -> MemoryType {
    if flags & flags::DEVICE != 0 {
        MemoryType::DeviceNGnRnE
    } else if flags & (flags::NO_CACHE | flags::WRITE_THROUGH) != 0 {
        MemoryType::NormalNC
    } else {
        MemoryType::NormalWB
    }
}
