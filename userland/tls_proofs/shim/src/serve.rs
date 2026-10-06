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

//! One request through the capsule's dispatch.

use alloc::vec::Vec;

use crate::protocol::{decode_request, encode_response, EINVAL};

const MAGIC: u32 = 0x4e4f_4358;
const REQUEST_ID: u32 = 7;

/// The v1 frame the kernel's crypto client builds around a syscall's body.
pub(crate) fn frame(op: u16, body: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(20 + body.len());
    out.extend_from_slice(&MAGIC.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&op.to_le_bytes());
    out.extend_from_slice(&[0, 0, 0, 0]);
    out.extend_from_slice(&REQUEST_ID.to_le_bytes());
    out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    out.extend_from_slice(body);
    out
}

/// What the capsule's runner does with one received message.
pub(crate) fn answer(message: &[u8]) -> Vec<u8> {
    match decode_request(message) {
        Ok(request) => crate::server::dispatch::dispatch(request),
        Err(_) => encode_response(0, 0, 0, EINVAL, &[]),
    }
}

/// A syscall the kernel forwards to the pool: frame it, serve it, and return
/// the body, or the status the pool answered with.
pub(crate) fn kernel_call(op: u16, body: &[u8]) -> Result<Vec<u8>, i64> {
    crate::count::note(|c| c.kernel_round_trips += 1);
    let reply = answer(&frame(op, body));
    if reply.len() < 24 {
        return Err(-71);
    }
    let status = i32::from_le_bytes([reply[20], reply[21], reply[22], reply[23]]);
    if status != 0 {
        return Err(status as i64);
    }
    Ok(reply[24..].to_vec())
}

/// Copy `bytes` to a caller's buffer and return the count, as a syscall does.
pub(crate) fn deliver(bytes: &[u8], out: *mut u8) -> i64 {
    unsafe { core::ptr::copy_nonoverlapping(bytes.as_ptr(), out, bytes.len()) };
    bytes.len() as i64
}

/// A caller's (pointer, length) as a slice.
pub(crate) fn input<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    if len == 0 {
        return &[];
    }
    unsafe { core::slice::from_raw_parts(ptr, len) }
}
