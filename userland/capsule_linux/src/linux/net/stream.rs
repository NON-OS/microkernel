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

//! Bytes on a stream that reaches outside the family, through net.sockets.

use alloc::vec::Vec;

use crate::linux::abi::errno;

use super::call::call;
use super::ops::{OP_CLOSE, OP_RECV, OP_SEND};

/// The most net.sockets carries in one call.
pub const MAX_IO: usize = 32 << 10;

pub fn send_bytes(handle: u32, bytes: &[u8]) -> u64 {
    let mut body = Vec::with_capacity(4 + bytes.len());
    body.extend_from_slice(&handle.to_le_bytes());
    body.extend_from_slice(bytes);
    match call(OP_SEND, &body, 0) {
        Some((0, _)) => errno::ok(bytes.len() as u64),
        Some(_) => errno::fail(errno::EPIPE),
        None => errno::fail(errno::EIO),
    }
}

/// Up to `want` bytes. net.sockets answers the same for a quiet stream, a
/// closed one and a reset one, so any refusal reads as ECONNRESET.
pub fn recv_bytes(handle: u32, want: usize) -> Result<Vec<u8>, u64> {
    let want = want.min(MAX_IO);
    let Some((status, mut bytes)) = call(OP_RECV, &handle.to_le_bytes(), want) else {
        return Err(errno::fail(errno::EIO));
    };
    if status != 0 {
        return Err(errno::fail(errno::ECONNRESET));
    }
    bytes.truncate(want);
    Ok(bytes)
}

pub fn close(handle: u32) {
    let _ = call(OP_CLOSE, &handle.to_le_bytes(), 0);
}
