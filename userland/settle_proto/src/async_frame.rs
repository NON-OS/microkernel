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

//! ASYNC (plan/28, v1.4.15, v1.4.23, v1.4.53): header flags bit 0, from
//! payment and market.index, and from the wallet for BROADCAST, READ and
//! INTENT only. The caller sends the request without waiting; settle
//! answers with mk_ipc_send_to_pid(caller), the same frame echoing op, flags
//! and id, into the caller's service inbox. Each believes an answer only from
//! settle's registry pid, with this wire's magic and its own request id.

use alloc::vec::Vec;

use crate::frame::{framed, reply_body, ReplyError};
use crate::ops::{OP_BROADCAST, OP_INTENT, OP_READ};

pub const FLAG_ASYNC: u16 = 1;
/// The names the registry gives the capsules that may ask ASYNC: the meter,
/// and the market for its bond cache.
pub const ASYNC_CALLERS: [&[u8]; 2] = [b"payment", b"market.index"];
/// The wallet windows, ASYNC for WALLET_ASYNC_OPS alone (v1.4.53).
pub const WALLET_ASYNC: [&[u8]; 3] =
    [b"app.nonos_wallet", b"app.nonos_wallet.1", b"app.nonos_wallet.2"];
pub const WALLET_ASYNC_OPS: [u16; 3] = [OP_BROADCAST, OP_READ, OP_INTENT];

pub fn request_async(op: u16, request_id: u32, body: &[u8]) -> Vec<u8> {
    framed(op, FLAG_ASYNC, request_id, &[body])
}

pub fn is_async(flags: u16) -> bool {
    flags & FLAG_ASYNC != 0
}

/// Whether `sender` may send `op` with `flags`: a plain call from anyone the
/// op admits; ASYNC from an ASYNC_CALLERS pid, or a wallet's for
/// WALLET_ASYNC_OPS.
pub fn admits_flags(
    flags: u16,
    op: u16,
    sender: u32,
    owner_of: impl Fn(&[u8]) -> Option<u32>,
) -> bool {
    let holds = |names: &[&[u8]]| sender != 0 && names.iter().any(|n| owner_of(n) == Some(sender));
    !is_async(flags)
        || holds(&ASYNC_CALLERS)
        || (WALLET_ASYNC_OPS.contains(&op) && holds(&WALLET_ASYNC))
}

/// An ASYNC answer's body, believed only from `settle_pid` and only as the
/// answer to `op` sent as `request_id`.
pub fn async_answer(
    rx: &[u8],
    sender: u32,
    settle_pid: u32,
    op: u16,
    request_id: u32,
) -> Result<&[u8], ReplyError> {
    if sender == 0 || sender != settle_pid {
        return Err(ReplyError::Foreign);
    }
    reply_body(rx, op, request_id)
}
