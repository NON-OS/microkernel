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

//! `OP_RX_BATCH`: every received frame waiting, in one numbered reply.
//!
//! One frame per round trip held net.core to two frames a poll. A batch is
//! kept until the caller asks for the next number: a frame off the ring is
//! gone, and a reply that reached a caller that had stopped waiting would
//! otherwise lose every frame in it. Same contract as virtio_net's.

extern crate alloc;

use alloc::vec::Vec;

use crate::constants::regs::REG_RDT;
use crate::protocol::{Request, BATCH_MAX_BYTES, BATCH_MAX_FRAMES, E_AGAIN, E_INVAL};
use crate::server::error::reply_with_status;
use crate::server::handlers::rx_send::send;
use crate::setup::Driver;

/// The last batch handed out, and to whom, kept until its number moves on.
pub struct Kept {
    pid: u32,
    seq: u32,
    body: Vec<u8>,
}

pub fn handle(
    sender: u32,
    driver: &mut Driver,
    req: &Request,
    body: &[u8],
    kept: &mut Option<Kept>,
    tx: &mut [u8],
) {
    // An empty request only asks whether batches are served: an empty batch.
    if body.is_empty() {
        return send(sender, req, &0u32.to_le_bytes());
    }
    let Ok(seq) = <[u8; 4]>::try_from(body).map(u32::from_le_bytes) else {
        return reply_with_status(sender, tx, req, E_INVAL);
    };
    if let Some(k) = kept.as_ref().filter(|k| k.pid == sender && k.seq == seq) {
        return send(sender, req, &k.body);
    }
    let regs = &driver.regs;
    // SAFETY: `regs` carries the broker MmioMap base for BAR0, RDT is a
    // 32-bit aligned offset in it, and every index comes from consume().
    let give_back = |idx: u16| unsafe { regs.w32(REG_RDT, u32::from(idx)) };
    let Some(batch) = driver.rx.take_batch(BATCH_MAX_FRAMES, BATCH_MAX_BYTES, give_back) else {
        *kept = None;
        return reply_with_status(sender, tx, req, E_AGAIN);
    };
    send(sender, req, &batch);
    *kept = Some(Kept { pid: sender, seq, body: batch });
}
