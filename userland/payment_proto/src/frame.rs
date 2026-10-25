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

//! Framing a request and a reply.

use alloc::vec::Vec;

use crate::bytes::len_u32;
use crate::header::{HDR_LEN, MAGIC, STATUS_LEN, VERSION};

#[path = "frame_reply.rs"]
mod frame_reply;
pub use frame_reply::{reply_body, ReplyError};

/// The status word's length, as the header writes it.
const STATUS_WORD: u32 = 4;
const _: () = assert!(STATUS_LEN == 4);

fn framed(op: u16, flags: u16, request_id: u32, parts: &[&[u8]]) -> Option<Vec<u8>> {
    let len = parts.iter().try_fold(0usize, |t, p| t.checked_add(p.len()))?;
    let mut out = head(op, flags, request_id, len_u32(len)?);
    parts.iter().for_each(|p| out.extend_from_slice(p));
    Some(out)
}

fn head(op: u16, flags: u16, request_id: u32, len: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity(HDR_LEN);
    out.extend_from_slice(&MAGIC.to_le_bytes());
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&op.to_le_bytes());
    out.extend_from_slice(&flags.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&request_id.to_le_bytes());
    out.extend_from_slice(&len.to_le_bytes());
    out
}

pub fn request(op: u16, request_id: u32, body: &[u8]) -> Option<Vec<u8>> {
    framed(op, 0, request_id, &[body])
}

/// The reply to a request with this `op`, `flags` and `request_id`.
pub fn reply(op: u16, flags: u16, request_id: u32, status: i32, body: &[u8]) -> Option<Vec<u8>> {
    framed(op, flags, request_id, &[&status.to_le_bytes(), body])
}

/// A reply that is its status alone. Its length is the status word's, which
/// always fits, so a server answers with it when its body would not frame.
pub fn status_reply(op: u16, flags: u16, request_id: u32, status: i32) -> Vec<u8> {
    let mut out = head(op, flags, request_id, STATUS_WORD);
    out.extend_from_slice(&status.to_le_bytes());
    out
}
