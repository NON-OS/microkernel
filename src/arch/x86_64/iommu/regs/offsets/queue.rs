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

//! The invalidation queue registers (VT-d 3.4, sections 6.5.2 and 11.4.9).

/// Invalidation Queue Head, Tail and Address. Head and tail hold a
/// descriptor index shifted left by four, the byte offset of a 128-bit
/// descriptor.
pub const IQH: usize = 0x080;
pub const IQT: usize = 0x088;
pub const IQA: usize = 0x090;

/// GCMD.QIE and its GSTS counterpart, bit 26. Persistent, so `gcmd_with`
/// carries it into every later command.
pub const GCMD_QIE: u32 = 1 << 26;
pub const GSTS_QIES: u32 = 1 << 26;

/// Fault Status bits the queue reports: a descriptor the unit refused, an
/// invalidation completion error, a device TLB invalidation that timed out.
pub const FSTS_IQE: u32 = 1 << 4;
pub const FSTS_ICE: u32 = 1 << 5;
pub const FSTS_ITE: u32 = 1 << 6;

/// One 4 KiB page of 128-bit descriptors: QS of zero, 256 entries.
pub const QUEUE_ENTRIES: u16 = 256;

/// IQA for a page-aligned queue base: 128-bit descriptors (DW clear) and a
/// queue size of one page (QS zero).
pub const fn iqa_value(base_phys: u64) -> u64 {
    base_phys & !0xFFF
}

/// The IQT or IQH value for descriptor `index`.
pub const fn queue_offset(index: u16) -> u64 {
    (index as u64) << 4
}

/// The descriptor index an IQH or IQT value names.
pub const fn queue_index(register: u64) -> u16 {
    ((register >> 4) & 0x7FFF) as u16
}

/// The slot after `index` in a queue of `QUEUE_ENTRIES`.
pub const fn queue_next(index: u16) -> u16 {
    (index + 1) % QUEUE_ENTRIES
}

/// Slots free for software to fill between the unit's `head` and software's
/// `tail`. One slot always stays empty: head equal to tail means an empty
/// queue, so a full one must stop a slot short.
pub const fn queue_free(head: u16, tail: u16) -> u16 {
    (head + QUEUE_ENTRIES - tail - 1) % QUEUE_ENTRIES
}
