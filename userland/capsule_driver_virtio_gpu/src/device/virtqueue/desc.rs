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

//! Descriptors, each written through the region's bounds checks.

use super::layout::QueueLayout;
use crate::constants::{VQ_DESC_OFFSET, VRING_DESC_F_NEXT, VRING_DESC_F_WRITE};

pub const DESC_LEN: usize = 16;

/// Descriptor `index`: `len` bytes at device address `addr`.
pub fn write_desc(layout: QueueLayout, index: u16, addr: u64, len: u32, flags: u16, next: u16) {
    let at = VQ_DESC_OFFSET + DESC_LEN * usize::from(index % layout.queue_size);
    let ring = layout.ring;
    ring.w32(at, addr as u32);
    ring.w32(at + 4, (addr >> 32) as u32);
    ring.w32(at + 8, len);
    ring.w16(at + 12, flags);
    ring.w16(at + 14, next);
}

/// A command: the request the device reads, chained to the buffer it
/// writes its answer in.
pub fn write_request_chain(
    layout: QueueLayout,
    head: u16,
    req_addr: u64,
    req_len: u32,
    resp_addr: u64,
    resp_len: u32,
) -> u16 {
    let resp_idx = head.wrapping_add(1) % layout.queue_size;
    write_desc(layout, head, req_addr, req_len, VRING_DESC_F_NEXT, resp_idx);
    write_desc(layout, resp_idx, resp_addr, resp_len, VRING_DESC_F_WRITE, 0);
    head
}
