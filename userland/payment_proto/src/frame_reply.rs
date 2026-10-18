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

//! Believing a reply: whose it is, and its status.

use crate::bytes::fixed;
use crate::header::{parse_header, STATUS_LEN};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ReplyError {
    /// Not this wire's header, or not whole.
    Foreign,
    /// Whole, but the answer to another call.
    Stale,
    /// A whole reply with a non-zero status.
    Status(i32),
}

/// The body after the status of `rx`, the reply to `op` sent as
/// `request_id`, or why it is not believed. A call that timed out can leave
/// its answer in flight to arrive as the next call's.
pub fn reply_body(rx: &[u8], op: u16, request_id: u32) -> Result<&[u8], ReplyError> {
    let (h, payload) = parse_header(rx).ok_or(ReplyError::Foreign)?;
    if h.op != op || h.request_id != request_id {
        return Err(ReplyError::Stale);
    }
    let status = payload.get(..STATUS_LEN).ok_or(ReplyError::Foreign)?;
    match i32::from_le_bytes(fixed(status, 0).ok_or(ReplyError::Foreign)?) {
        0 => payload.get(STATUS_LEN..).ok_or(ReplyError::Foreign),
        s => Err(ReplyError::Status(s)),
    }
}
