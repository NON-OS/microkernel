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

//! Where a queue is notified.
//!
//! The specification puts queue N's doorbell at
//! notify base + queue_notify_off * notify_off_multiplier, both factors
//! read from the device. The product is worked out in 64 bits with checked
//! arithmetic and the 16-bit write it is for must land inside the part of
//! the notify region that is mapped, on a 2-byte boundary; otherwise the
//! queue has no usable doorbell and setup stops.

const NOTIFY_WRITE_BYTES: u64 = 2;

/// Byte offset of the doorbell inside the mapped notify region, or `None`
/// when it would fall outside `mapped_len` or off a 16-bit boundary.
pub fn notify_offset(queue_notify_off: u16, multiplier: u32, mapped_len: usize) -> Option<usize> {
    let off = (queue_notify_off as u64).checked_mul(multiplier as u64)?;
    let end = off.checked_add(NOTIFY_WRITE_BYTES)?;
    if end > mapped_len as u64 || !off.is_multiple_of(NOTIFY_WRITE_BYTES) {
        return None;
    }
    usize::try_from(off).ok()
}
