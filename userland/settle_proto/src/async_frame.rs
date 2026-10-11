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

//! ASYNC: header flags bit 0, from payment only. The
//! meter sends the request without waiting; settle answers with
//! mk_ipc_send_to_pid(caller), the same frame echoing op, flags and id,
//! into the meter's service inbox. The meter believes an answer only from
//! settle's registry pid, with this wire's magic and its own request id.

use alloc::vec::Vec;

use crate::frame::{framed, reply_body, ReplyError};

pub const FLAG_ASYNC: u16 = 1;
/// The name the registry gives the one capsule that may ask ASYNC.
pub const ASYNC_CALLER: &[u8] = b"payment";

pub fn request_async(op: u16, request_id: u32, body: &[u8]) -> Vec<u8> {
    framed(op, FLAG_ASYNC, request_id, &[body])
}

pub fn is_async(flags: u16) -> bool {
    flags & FLAG_ASYNC != 0
}

/// Whether `sender` may send with `flags`: a plain call from anyone the op
/// admits, ASYNC from payment's pid alone.
pub fn admits_flags(flags: u16, sender: u32, owner_of: impl Fn(&[u8]) -> Option<u32>) -> bool {
    !is_async(flags) || (sender != 0 && owner_of(ASYNC_CALLER) == Some(sender))
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
