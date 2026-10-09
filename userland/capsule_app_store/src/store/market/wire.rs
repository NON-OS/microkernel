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

//! One call to the market service. The framing, and the checks on what
//! comes back, are `nonos_market_proto`'s, shared with the Terminal's
//! `market` command.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_ipc_call_timeout;
use nonos_market_proto::{reply_body, request};

use super::failure::Failure;

/// A catalogue reply carries every listing, so this is sized for the
/// catalogue rather than for one entry.
const RX_CAP: usize = 96 << 10;

/// Long enough for the capsule to walk its index, short enough that a
/// service that has stopped answering does not freeze a repaint.
const TIMEOUT_MS: u64 = 1500;

/// The reply's body after its status word, or why there is none. A reply
/// that is not the market's, or answers another call, is never read.
pub fn exchange(port: u32, op: u16, request_id: u32, body: &[u8]) -> Result<Vec<u8>, Failure> {
    if port == 0 {
        return Err(Failure::NoReply);
    }
    let tx = request(op, request_id, body);
    let mut rx = vec![0u8; RX_CAP];
    let rc = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        TIMEOUT_MS,
    );
    let got = usize::try_from(rc).map_err(|_| Failure::NoReply)?.min(rx.len());
    reply_body(&rx[..got], op, request_id).map(<[u8]>::to_vec).map_err(Failure::from)
}
