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

//! One command, polled to completion. A command the device does not answer
//! by the deadline, or answers out of turn, ends in a device reset and the
//! queue out of service, by name. A timed-out command used to stay
//! published while the next one rewrote its staging page and descriptors,
//! so the device could run the stale chain with the new bytes.

use super::control_queue::ControlQueue;
use super::{avail, desc, used, wait};
use crate::clock::expired_after;
use crate::constants::{CTRLQ_INDEX, CTRLQ_MS, VQ_STAGING_OFFSET};

pub struct SubmitOutput {
    pub used_len: u32,
}

pub fn submit_sync(
    q: &ControlQueue,
    request: &[u8],
    resp_len: u32,
) -> Result<SubmitOutput, &'static str> {
    if q.is_reset() {
        return Err("virtio-gpu: control queue reset after a stalled command");
    }
    let layout = q.layout;
    let req_len = request.len();
    if req_len == 0 || resp_len == 0 {
        return Err("virtio-gpu: empty request/response");
    }
    let total = req_len.checked_add(resp_len as usize).ok_or("virtio-gpu: overflow")?;
    if total > layout.staging_len() {
        return Err("virtio-gpu: staging too small");
    }
    for at in 0..total {
        layout.ring.w8(VQ_STAGING_OFFSET + at, request.get(at).copied().unwrap_or(0));
    }
    let req_addr = layout.staging_device_addr();
    let resp_addr = req_addr + req_len as u64;
    let next = q.next.get();
    let head = next % layout.queue_size;
    desc::write_request_chain(layout, head, req_addr, req_len as u32, resp_addr, resp_len);
    avail::publish(layout, next, head);
    q.regs.notify(CTRLQ_INDEX);
    if let Err(e) = wait::used_moved(layout, next, expired_after(CTRLQ_MS)) {
        q.reset_device();
        return Err(e);
    }
    let entry = used::read_entry(layout, next);
    let done = next.wrapping_add(1);
    q.next.set(done);
    // One command was in flight, so the device may have used exactly it.
    // The whole 32-bit id: cut to 16 bits, an element naming head + 65536
    // (a descriptor no ring this size holds) passed for the one published.
    if used::read_idx(layout) != done || entry.id != u32::from(head) {
        q.reset_device();
        return Err("virtio-gpu: used id mismatch");
    }
    Ok(SubmitOutput { used_len: entry.len })
}
