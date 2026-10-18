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

//! One read, write or flush, answered before its command is recovered:
//! recovery can take minutes (an Abort, its grace, a disable and an enable
//! on CAP.TO each, the host memory buffer and the queues again), and the
//! kernel stops waiting 35 s after it queued a request. The command stays
//! out, its buffer untouched, until recovery retires it, and a request
//! already queued when the server next takes one is bounded by the time of
//! this reply, or by when another sender's stalled request was taken
//! (nvm::bound).

use super::plan::recover;
use super::stalled::{Serving, Stalled};
use super::state::Recovered;
use crate::error::NvmeResult;
use crate::log::{emit, Line};
use crate::nvm::IoQueue;
use crate::protocol::E_NODEV;
use crate::regs::Regs;

/// Run `op` on the I/O queue (retried, for the pid `sender`) and
/// `reply` with Ok or the reply status, both handed `tx`, the buffer a read
/// fills; then recover a command still out. A queue that will not serve
/// again is dropped, which unmaps nothing (dma::DmaRegion): later requests
/// are answered E_NODEV.
pub fn io_request<T: ?Sized, F, R>(on: Serving<'_>, sender: u32, tx: &mut T, mut op: F, reply: R)
where
    F: FnMut(&mut IoQueue, Regs, &mut T) -> NvmeResult<()>,
    R: FnOnce(&mut T, Result<(), i32>),
{
    let Serving { device, admin, io, hmb } = on;
    let regs = device.regs;
    let Some(queue) = io.as_mut() else { return reply(tx, Err(E_NODEV)) };
    let done = queue.retried(sender, |q| op(q, regs, tx));
    // Bounded from just before the reply goes: never later than the kernel.
    let stuck = queue.stuck();
    if stuck.is_some() {
        queue.bound_next(sender);
    }
    reply(tx, done.map_err(|e| queue.failure_status(e)));
    let Some(cid) = stuck else { return };
    let ended = recover(&mut Stalled { device, admin, io: queue, hmb });
    if !ended.serves() {
        *io = None;
    }
    say(cid, ended);
}

fn say(cid: u16, ended: Recovered) {
    let mut line = Line::new();
    line.text(b"I/O command ").hex(cid as u64, 4).text(b" got no completion in time: ");
    line.text(ended.words().as_bytes());
    emit(&mut line);
}
