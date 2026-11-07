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

//! Many commands on the control queue at once. Each sits in its own slot
//! of the staging page with its own descriptor pair; all are published
//! together, the device is notified once and waited for once. The next
//! command, batched or not, starts on a ring the device has emptied, so
//! the descriptors are free to reuse from 0.

use super::control_queue::ControlQueue;
use super::{avail, desc, used, wait};
use crate::clock::expired_after;
use crate::constants::{CTRLQ_INDEX, CTRLQ_MS, VQ_STAGING_OFFSET};

const SLOT_ALIGN: usize = 16;
/// Commands one batch may hold: the used ids are checked off in a u128.
pub const MAX_BATCH: usize = 128;

/// One command's room in the staging page: request, then its answer.
pub const fn slot_len(req: usize, resp: usize) -> usize {
    (req + resp).next_multiple_of(SLOT_ALIGN)
}

/// How many commands of `slot` bytes fit one batch.
pub fn capacity(q: &ControlQueue, slot: usize) -> usize {
    let pairs = usize::from(q.layout.queue_size) / 2;
    pairs.min(q.layout.staging_len() / slot.max(1)).min(MAX_BATCH)
}

/// Every request of `reqs` sent, as many to a batch as fit; `ok` judges
/// each answer. A refusal fails the whole once its batch is answered in
/// full, so the ring stays in step with the device.
pub fn submit_all<const R: usize, const A: usize>(
    q: &ControlQueue,
    reqs: &mut dyn Iterator<Item = [u8; R]>,
    ok: &dyn Fn(&[u8; A]) -> bool,
) -> Result<(), &'static str> {
    let slot = slot_len(R, A);
    let cap = capacity(q, slot);
    if cap == 0 {
        return Err("virtio-gpu: staging too small");
    }
    let mut refused = false;
    loop {
        if q.is_reset() {
            return Err("virtio-gpu: control queue reset after a stalled command");
        }
        let n = stage(q, reqs, (slot, A), cap);
        if n == 0 {
            break;
        }
        refused |= !run(q, n, (slot, R), ok)?;
    }
    match refused {
        true => Err("virtio-gpu: a batched command was rejected"),
        false => Ok(()),
    }
}

/// Up to `cap` requests written to their slots and chained; how many.
fn stage<const R: usize>(
    q: &ControlQueue,
    reqs: &mut dyn Iterator<Item = [u8; R]>,
    (slot, resp): (usize, usize),
    cap: usize,
) -> usize {
    let (layout, next) = (q.layout, q.next.get());
    let mut n = 0;
    while n < cap {
        let Some(req) = reqs.next() else { break };
        let at = VQ_STAGING_OFFSET + n * slot;
        for (i, b) in req.iter().chain(core::iter::repeat_n(&0, resp)).enumerate() {
            layout.ring.w8(at + i, *b);
        }
        let req_addr = layout.staging_device_addr() + (n * slot) as u64;
        let head = (2 * n) as u16;
        desc::write_request_chain(
            layout,
            head,
            req_addr,
            R as u32,
            req_addr + R as u64,
            resp as u32,
        );
        avail::put(layout, next.wrapping_add(n as u16), head);
        n += 1;
    }
    n
}

/// The `n` staged commands published, notified once and waited for;
/// whether every answer was `ok`.
fn run<const A: usize>(
    q: &ControlQueue,
    n: usize,
    (slot, req): (usize, usize),
    ok: &dyn Fn(&[u8; A]) -> bool,
) -> Result<bool, &'static str> {
    let (layout, next) = (q.layout, q.next.get());
    let done = next.wrapping_add(n as u16);
    avail::publish_to(layout, done);
    q.regs.notify(CTRLQ_INDEX);
    if let Err(e) = wait::used_reached(layout, next, n as u16, expired_after(CTRLQ_MS)) {
        q.reset_device();
        return Err(e);
    }
    // Each head named once, and no more answered than were sent.
    let mut seen = 0u128;
    for k in 0..n as u16 {
        let id = used::read_entry(layout, next.wrapping_add(k)).id as usize;
        let ours = id.is_multiple_of(2) && id / 2 < n && seen & (1 << (id / 2)) == 0;
        if !ours {
            q.reset_device();
            return Err("virtio-gpu: used id mismatch");
        }
        seen |= 1 << (id / 2);
    }
    q.next.set(done);
    if used::read_idx(layout) != done {
        q.reset_device();
        return Err("virtio-gpu: used id mismatch");
    }
    let mut all = true;
    for k in 0..n {
        let mut answer = [0u8; A];
        q.read_response(k * slot + req, &mut answer);
        all &= ok(&answer);
    }
    Ok(all)
}
