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

use nonos_libc::mk_yield;

use super::constants::LEG_QUEUE_NOTIFY;
use super::queue::Queue;
use super::regs::Regs;

const MAX_YIELDS: u32 = 100_000;

/// One request, polled to completion. No interrupt is involved: the device
/// has Interrupt Disable set (see `setup::irq`). The used ring is the only
/// completion signal; an interrupt sequence change is not one, since on a
/// shared line it may belong to another device and the buffer would be
/// read before the device wrote it.
pub fn fill(regs: Regs, queue: &mut Queue) -> Result<u32, &'static str> {
    queue.post_request();
    unsafe {
        regs.w16(LEG_QUEUE_NOTIFY, 0);
    }

    let target = queue.last_used.wrapping_add(1);

    let mut tries = 0u32;
    while queue.used_idx() != target {
        if tries >= MAX_YIELDS {
            return Err("virtio-rng: device did not respond");
        }
        let _ = mk_yield();
        tries = tries.wrapping_add(1);
    }
    queue.last_used = target;
    Ok(queue.used_len())
}
