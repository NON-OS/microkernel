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

//! Programming one legacy virtqueue.
//!
//! A legacy device cannot be told a ring size: QueueNum is read-only, and
//! the device lays out descriptors, avail and used rings for its own
//! QueueNumMax from the one page number it is given. The driver's grant and
//! offsets fit a 256-entry ring and nothing else (at 512 the used ring
//! starts where the grant ends; at 1024, bhyve's size, all of it lies in
//! someone else's memory), so any other size is refused, and the device
//! marked FAILED, before the page number is written.

use crate::constants::{
    LEG_QUEUE_NUM, LEG_QUEUE_PFN, LEG_QUEUE_SEL, LEG_STATUS, RING_SLOTS, STATUS_FAILED,
};
use crate::regs::Regs;

/// Point queue `queue_index` at the 256-entry ring at `queue_phys`; the
/// number of buffers to prime, at most `queue_size_hint`.
pub fn program_queue(
    regs: Regs,
    queue_index: u16,
    queue_phys: u64,
    queue_size_hint: u16,
) -> Result<u16, &'static str> {
    let refuse = |why| {
        regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_FAILED);
        Err(why)
    };
    if queue_phys & 0xFFF != 0 {
        return refuse("virtio-net: unaligned queue");
    }
    let Ok(pfn) = u32::try_from(queue_phys >> 12) else {
        return refuse("virtio-net: queue pfn overflow");
    };
    regs.w16(LEG_QUEUE_SEL, queue_index);
    match regs.r16(LEG_QUEUE_NUM) {
        0 => return refuse("virtio-net: queue missing"),
        RING_SLOTS => {}
        _ => return refuse("virtio-net: legacy queue size is not 256"),
    }
    regs.w32(LEG_QUEUE_PFN, pfn);
    Ok(core::cmp::min(RING_SLOTS, queue_size_hint))
}
