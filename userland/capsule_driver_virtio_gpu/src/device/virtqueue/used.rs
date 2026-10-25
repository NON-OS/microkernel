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

//! The used ring, which the device writes: every value read here is the
//! device's word and is checked by whoever uses it.

use super::layout::QueueLayout;
use crate::constants::VQ_USED_OFFSET;

const USED_IDX: usize = VQ_USED_OFFSET + 2;
const USED_RING: usize = VQ_USED_OFFSET + 4;
const USED_ELEM_SIZE: usize = 8;

/// How many chains the device has used, ever (it wraps).
pub fn read_idx(layout: QueueLayout) -> u16 {
    layout.ring.r16(USED_IDX)
}

pub struct UsedEntry {
    pub id: u32,
    pub len: u32,
}

/// Used entry `n`, once an index that covers it has been seen.
pub fn read_entry(layout: QueueLayout, n: u16) -> UsedEntry {
    let at = USED_RING + USED_ELEM_SIZE * usize::from(n % layout.queue_size);
    // Order these reads after the used index that proved them written, as
    // the device wrote them (libc's barrier, not a fence, on aarch64).
    nonos_libc::dma_rmb();
    UsedEntry { id: layout.ring.r32(at), len: layout.ring.r32(at + 4) }
}
