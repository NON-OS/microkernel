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

//! Which replies are believed.
//!
//! A reply is read only when it is the market's (its magic and version),
//! answers this call (its op and request id), and carries the whole payload
//! its header names. A call that timed out can leave its answer in flight,
//! to arrive as the reply to the next call: without the id check, one
//! listing's detail would be painted under another listing's name.

use super::header::{HDR_LEN, MAGIC, STATUS_LEN, VERSION};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ReplyError {
    /// Fewer bytes than a header and a status.
    Short,
    /// Not the market's magic or version.
    Foreign,
    /// The market's, but the answer to another call: its op or request id
    /// is not this call's.
    Stale,
    /// The header names more payload than arrived, or less than a status.
    Truncated,
    /// A whole reply with a non-zero status.
    Status(i32),
}

/// The body after the status word of `rx`, the reply to `op` sent with
/// `request_id`, or why it is not believed.
pub fn reply_body(rx: &[u8], op: u16, request_id: u32) -> Result<&[u8], ReplyError> {
    if rx.len() < HDR_LEN + STATUS_LEN {
        return Err(ReplyError::Short);
    }
    let u16_at = |at: usize| u16::from_le_bytes([rx[at], rx[at + 1]]);
    let u32_at = |at: usize| u32::from_le_bytes([rx[at], rx[at + 1], rx[at + 2], rx[at + 3]]);
    if u32_at(0) != MAGIC || u16_at(4) != VERSION {
        return Err(ReplyError::Foreign);
    }
    if u16_at(6) != op || u32_at(12) != request_id {
        return Err(ReplyError::Stale);
    }
    let payload = u32_at(16) as usize;
    let end = HDR_LEN.checked_add(payload).ok_or(ReplyError::Truncated)?;
    if payload < STATUS_LEN || end > rx.len() {
        return Err(ReplyError::Truncated);
    }
    match u32_at(HDR_LEN) as i32 {
        0 => Ok(&rx[HDR_LEN + STATUS_LEN..end]),
        status => Err(ReplyError::Status(status)),
    }
}
