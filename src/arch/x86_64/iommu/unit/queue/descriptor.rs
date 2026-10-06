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

//! The 128-bit invalidation descriptors the queue carries (VT-d 3.4, section
//! 6.5.2), low quadword first. Encodings match Linux include/linux/intel-iommu.h
//! (QI_CC_*, QI_IOTLB_*, QI_IEC_*, QI_IWD_*).

pub type Descriptor = [u64; 2];

const TYPE_CONTEXT: u64 = 0x1;
const TYPE_IOTLB: u64 = 0x2;
const TYPE_IEC: u64 = 0x4;
const TYPE_WAIT: u64 = 0x5;

/// Granularity field, bits 5:4, value 01: every entry the unit caches.
const GRANULARITY_GLOBAL: u64 = 1 << 4;

/// Context-cache invalidation, 6.5.2.1: drop every cached context entry.
pub const fn context_global() -> Descriptor {
    [TYPE_CONTEXT | GRANULARITY_GLOBAL, 0]
}

/// IOTLB invalidation, 6.5.2.3: drop every cached translation and paging
/// structure entry. DR (bit 7) and DW (bit 6) drain reads and writes still
/// in flight, when CAP says the unit can.
pub const fn iotlb_global(drain_reads: bool, drain_writes: bool) -> Descriptor {
    let dr = if drain_reads { 1 << 7 } else { 0 };
    let dw = if drain_writes { 1 << 6 } else { 0 };
    [TYPE_IOTLB | GRANULARITY_GLOBAL | dr | dw, 0]
}

/// Interrupt entry cache invalidation, 6.5.2.7, global: G (bit 4) clear.
pub const fn iec_global() -> Descriptor {
    [TYPE_IEC, 0]
}

/// Interrupt entry cache invalidation for the 2^`mask` entries from `index`:
/// G set, IM in bits 31:27, IIDX in bits 47:32.
pub const fn iec_index(index: u16, mask: u8) -> Descriptor {
    [TYPE_IEC | (1 << 4) | (((mask & 0x1F) as u64) << 27) | ((index as u64) << 32), 0]
}

/// Invalidation wait, 6.5.2.8: once every earlier descriptor is done, write
/// `data` to the dword at `status_phys`. SW (bit 5) asks for that write,
/// which the unit makes only after the earlier descriptors complete. FN (bit
/// 6) also holds later descriptors back, so a batch queued behind a wait
/// that timed out cannot overtake it.
pub const fn wait(status_phys: u64, data: u32) -> Descriptor {
    [TYPE_WAIT | (1 << 5) | (1 << 6) | ((data as u64) << 32), status_phys & !0x3]
}
