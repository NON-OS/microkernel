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

//! The two user address windows the broker owns in every capsule: device
//! registers (MMIO grants) and DMA buffers. Pure, and proven on the host
//! (kernel_proofs::device_windows).
//!
//! Pages in these windows are mapped by the broker and are only ever given
//! back by it: MkMmioUnmap, MkDmaUnmap, MkDeviceRelease, or the capsule's
//! exit. The generic memory calls must not touch them. MkMunmap frees the
//! frame behind every page it unmaps, and here that frame is a device's
//! register page or a buffer the device (and its IOMMU domain) still
//! reaches: unmapped that way, the frame went to the allocator while the
//! grant stayed live, so the next owner's memory was the device's to read
//! and write, and a register page was zeroed through the directmap and put
//! on the RAM free list.

pub(super) const USER_MMIO_BASE: u64 = 0x0000_0080_0000_0000;
pub(super) const USER_MMIO_END: u64 = 0x0000_0090_0000_0000;
pub(super) const USER_DMA_BASE: u64 = 0x0000_00A0_0000_0000;
pub(super) const USER_DMA_END: u64 = 0x0000_00B0_0000_0000;

/// True if `[addr, addr + len)` overlaps either window. A range that wraps
/// the address space overlaps everything.
pub const fn touches_device_window(addr: u64, len: u64) -> bool {
    let end = match addr.checked_add(len) {
        Some(e) => e,
        None => return true,
    };
    overlaps(addr, end, USER_MMIO_BASE, USER_MMIO_END)
        || overlaps(addr, end, USER_DMA_BASE, USER_DMA_END)
}

const fn overlaps(start: u64, end: u64, lo: u64, hi: u64) -> bool {
    start < hi && end > lo
}
