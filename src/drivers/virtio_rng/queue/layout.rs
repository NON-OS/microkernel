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

//! The legacy virtqueue's layout for the size the device reports (L8).
//! Legacy virtio fixes the layout from the queue size: the descriptor
//! table, the available ring right after it, and the used ring at the next
//! 4096-byte boundary (virtio 1.x, the legacy interface's note on virtqueue
//! layout; recalled, not checked against the text here). The driver used a
//! size of its own, so a device of another size read its ring elsewhere.
//! Pure, so kernel_proofs checks it by path.

pub(crate) const QUEUE_ALIGN: usize = 4096;
/// The largest queue this driver keeps: three pages of ring.
pub(crate) const MAX_QUEUE: u16 = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Layout {
    /// Byte offset of the available ring: flags, idx, then one u16 a slot.
    pub(crate) avail: usize,
    /// Byte offset of the used ring: flags, idx, then (id u32, len u32) a slot.
    pub(crate) used: usize,
    /// Bytes the whole ring takes.
    pub(crate) bytes: usize,
}

/// None for a size the spec does not allow (0, not a power of two) or one
/// past `MAX_QUEUE`.
pub(crate) const fn layout(size: u16) -> Option<Layout> {
    if size == 0 || size > MAX_QUEUE || !size.is_power_of_two() {
        return None;
    }
    let n = size as usize;
    let avail = 16 * n;
    // flags, idx, ring[n], used_event
    let avail_end = avail + 2 * (3 + n);
    let used = avail_end.div_ceil(QUEUE_ALIGN) * QUEUE_ALIGN;
    // flags, idx, ring[n] of 8 bytes, avail_event
    Some(Layout { avail, used, bytes: used + 2 * 3 + 8 * n })
}
