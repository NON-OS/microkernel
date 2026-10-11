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

//! `OP_RX_BATCH`, as virtio-net serves it. A batch is kept until the caller
//! asks for the next number: taking frames off the ring is final, and a reply
//! that reached a caller that had stopped waiting would otherwise lose them.

extern crate alloc;

use alloc::vec::Vec;

use crate::protocol::{Request, E_AGAIN, E_INVAL};
use crate::server::error::reply_with_status;
use crate::server::handlers::rx_fill::fill;
use crate::server::handlers::rx_send::send;
use crate::setup::Driver;

/// The last batch handed out, and to whom, kept until its number moves on.
pub struct Kept {
    pub sender: u32,
    pub seq: u32,
    pub body: Vec<u8>,
}

pub fn handle(
    sender: u32,
    driver: &mut Driver,
    req: &Request,
    body: &[u8],
    kept: &mut Option<Kept>,
    tx: &mut [u8],
) -> bool {
    // An empty request only asks whether batches are served.
    if body.is_empty() {
        return send(sender, req, &0u32.to_le_bytes());
    }
    let Some(seq) = body.get(..4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]])) else {
        return reply_with_status(sender, tx, req, E_INVAL);
    };
    if let Some(k) = kept.as_ref().filter(|k| k.sender == sender && k.seq == seq) {
        return send(sender, req, &k.body);
    }
    let Some(batch) = fill(&mut driver.rx) else {
        *kept = None;
        return reply_with_status(sender, tx, req, E_AGAIN);
    };
    let sent = send(sender, req, &batch);
    *kept = Some(Kept { sender, seq, body: batch });
    sent
}
