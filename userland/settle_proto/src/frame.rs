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

//! Framing a request and a reply, and believing a reply.

use alloc::vec::Vec;

use crate::bytes::{fixed, len_u32};
use crate::header::{parse_header, HDR_LEN, MAGIC, STATUS_LEN, VERSION};

pub(crate) fn framed(op: u16, flags: u16, request_id: u32, parts: &[&[u8]]) -> Vec<u8> {
    let len = parts.iter().fold(0usize, |t, p| t.saturating_add(p.len()));
    let mut out = Vec::with_capacity(HDR_LEN.saturating_add(len));
    out.extend_from_slice(&MAGIC.to_le_bytes());
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&op.to_le_bytes());
    out.extend_from_slice(&flags.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&request_id.to_le_bytes());
    out.extend_from_slice(&len_u32(len).to_le_bytes());
    parts.iter().for_each(|p| out.extend_from_slice(p));
    out
}

pub fn request(op: u16, request_id: u32, body: &[u8]) -> Vec<u8> {
    framed(op, 0, request_id, &[body])
}

pub fn reply(op: u16, flags: u16, request_id: u32, status: i32, body: &[u8]) -> Vec<u8> {
    framed(op, flags, request_id, &[&status.to_le_bytes(), body])
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ReplyError {
    /// Not this wire's header, or not whole.
    Foreign,
    /// Whole, but the answer to another call.
    Stale,
    /// A whole reply with a non-zero status.
    Status(i32),
}

/// The body after the status of `rx`, the reply to `op` sent as `request_id`.
pub fn reply_body(rx: &[u8], op: u16, request_id: u32) -> Result<&[u8], ReplyError> {
    let (h, payload) = parse_header(rx).ok_or(ReplyError::Foreign)?;
    if h.op != op || h.request_id != request_id {
        return Err(ReplyError::Stale);
    }
    let status = fixed::<4>(payload, 0).ok_or(ReplyError::Foreign)?;
    match i32::from_le_bytes(status) {
        0 => payload.get(STATUS_LEN..).ok_or(ReplyError::Foreign),
        s => Err(ReplyError::Status(s)),
    }
}
