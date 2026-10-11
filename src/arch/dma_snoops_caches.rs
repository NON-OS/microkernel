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

/// Whether every device's DMA snoops the CPU's caches, so a buffer the CPU
/// wrote through a write-back mapping is what the device reads.
///
/// x86_64: yes, for every request without the No Snoop attribute, which a
/// function sends only once its driver sets Enable No Snoop (PCIe Base
/// Specification, Device Control register). aarch64 and
/// riscv64: not promised; a device tree says `dma-coherent` per node, and
/// boards such as the Raspberry Pi 4 have masters that do not snoop. The
/// broker then writes every grant out of the caches before a device sees it.
pub(crate) const fn dma_snoops_caches() -> bool {
    cfg!(target_arch = "x86_64")
}
