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

use alloc::vec;
use alloc::vec::Vec;
use spin::Mutex;

use super::batch::Joiner;
use super::session::session;
use crate::ipc::{call, CallError, OP_RECV, OP_RECV_BATCH};

/// What net.nym answers when nothing has come back yet. Not a failure: a
/// mixnet reply is seconds behind the request that asked for it, and treating
/// an empty queue as an error would close the tunnel on the way there.
const E_RX_EMPTY: u16 = 10;

/// What net.nym answers an op it does not know: one built before batched
/// reads, which is read a message at a time instead.
const E_BAD_OP: u16 = 3;

/// What net.nym answers for a session it no longer holds: it dropped every
/// session when its gateway link was lost and it dialled another.
pub const E_NO_SESSION: u16 = 8;

pub enum Delivery {
    /// The messages the mixnet delivered, each whole.
    Messages(Vec<Vec<u8>>),
    /// Nothing has arrived yet. Ask again.
    Empty,
    /// The transport is gone, so waiting longer will not help.
    Gone,
}

/// A message whose front came in one answer and whose rest is still to come.
static JOINER: Mutex<Joiner> = Mutex::new(Joiner::new());

/// Take everything the mixnet has delivered that fits in one answer,
/// letting net.nym wait at most `wait_ms` on an empty link.
///
/// One message per call made an answer that arrived as sixty messages cost
/// sixty round trips, and a reader asking once per frame of its own drew
/// one message per frame: the page trickled in at a packet a tick.
pub fn recv_batch(wait_ms: u32) -> Delivery {
    let Some(id) = session() else { return Delivery::Gone };
    let mut body = [0u8; 8];
    body[..4].copy_from_slice(&id.to_le_bytes());
    body[4..].copy_from_slice(&wait_ms.to_le_bytes());
    match call(OP_RECV_BATCH, &body) {
        Ok(answer) => {
            let unpacked = JOINER.lock().feed(&answer);
            if unpacked.malformed {
                crate::server::trace_step(b"mixnet answer malformed, bytes", answer.len() as u64);
            }
            if unpacked.dropped > 0 {
                crate::server::trace_step(b"mixnet pieces dropped", unpacked.dropped as u64);
            }
            Delivery::Messages(unpacked.messages)
        }
        Err(CallError::Remote(E_RX_EMPTY)) => Delivery::Empty,
        Err(CallError::Remote(E_BAD_OP)) => recv_once(),
        // Asking again under the same number would be refused for ever, and
        // every stream bound to it waited out its reader's patience. The
        // number is dropped so the next connection opens a session afresh.
        Err(CallError::Remote(E_NO_SESSION)) => {
            super::session::reset_session();
            Delivery::Gone
        }
        // A call that did not come back says nothing about the tunnel. The
        // capsule may simply have been busy on a gateway read, so this waits
        // rather than tearing a live connection down.
        Err(CallError::Transport) => Delivery::Empty,
        Err(_) => Delivery::Gone,
    }
}

/// Take one message off the session, if the mixnet has delivered one.
fn recv_once() -> Delivery {
    let Some(id) = session() else { return Delivery::Gone };
    match call(OP_RECV, &id.to_le_bytes()) {
        Ok(body) => Delivery::Messages(vec![body]),
        Err(CallError::Remote(E_RX_EMPTY)) => Delivery::Empty,
        Err(CallError::Transport) => Delivery::Empty,
        Err(_) => Delivery::Gone,
    }
}

/// Forget a message part way through; the session it came on is gone.
pub fn forget_partial() {
    JOINER.lock().reset();
}
