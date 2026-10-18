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
//! kept until the caller asks for the next number, since a frame read off
//! the part is gone; same contract as virtio_net's. A receive error after
//! some frames ends the batch early and is answered with E_IO on the next
//! new number, so it is reported rather than lost with the frames around it.

extern crate alloc;

use alloc::vec::Vec;

use crate::protocol::{Request, BATCH_MAX_BYTES, BATCH_MAX_FRAMES, E_AGAIN, E_INVAL, E_IO};
use crate::rx::batch::gather;
use crate::rx::recv_one;
use crate::server::error::reply_with_status;
use crate::server::handlers::rx_send::send;
use crate::setup::Driver;

/// The last batch handed out, to whom, and an error still owed.
#[derive(Default)]
pub struct Batches {
    kept: Option<(u32, u32, Vec<u8>)>,
    owed_error: bool,
}

pub fn handle(
    s: u32,
    driver: &mut Driver,
    req: &Request,
    body: &[u8],
    b: &mut Batches,
    tx: &mut [u8],
) {
    // An empty request only asks whether batches are served: an empty batch.
    if body.is_empty() {
        return send(s, req, &0u32.to_le_bytes());
    }
    let Ok(seq) = <[u8; 4]>::try_from(body).map(u32::from_le_bytes) else {
        return reply_with_status(s, tx, req, E_INVAL);
    };
    if let Some((_, _, kept)) = b.kept.as_ref().filter(|k| k.0 == s && k.1 == seq) {
        return send(s, req, kept);
    }
    b.kept = None;
    if core::mem::take(&mut b.owed_error) {
        return reply_with_status(s, tx, req, E_IO);
    }
    let got = gather(BATCH_MAX_FRAMES, BATCH_MAX_BYTES, |out| recv_one(driver, out));
    match got.body {
        Some(frames) => {
            send(s, req, &frames);
            b.owed_error = got.error.is_some();
            b.kept = Some((s, seq, frames));
        }
        None if got.error.is_some() => reply_with_status(s, tx, req, E_IO),
        None => reply_with_status(s, tx, req, E_AGAIN),
    }
}
