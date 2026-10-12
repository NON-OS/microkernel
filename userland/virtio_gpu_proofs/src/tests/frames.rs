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

//! Frames a client can put in driver.virtio_gpu0's inbox, and what must hold
//! for each: it decodes to the request and exact body it carries, or it
//! draws one whole reply naming the op, flags and request id it carried.

use crate::protocol::HDR_LEN;

pub const MAGIC: u32 = 0x4E56_4750;
pub const VERSION: u16 = 1;
/// The longest body the driver's receive buffer holds after the header.
pub const BODY_MAX: usize = crate::protocol::IPC_PAYLOAD_MAX;
/// The last op the driver knows.
pub const LAST_OP: u16 = crate::protocol::OP_GET_PRIMARY_SURFACE;

pub fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

pub fn frame(op: u16, flags: u16, request_id: u32, payload_len: u32, body: &[u8]) -> Vec<u8> {
    let mut f = Vec::with_capacity(HDR_LEN + body.len());
    f.extend_from_slice(&MAGIC.to_le_bytes());
    f.extend_from_slice(&VERSION.to_le_bytes());
    f.extend_from_slice(&op.to_le_bytes());
    f.extend_from_slice(&flags.to_le_bytes());
    f.extend_from_slice(&[0, 0]);
    f.extend_from_slice(&request_id.to_le_bytes());
    f.extend_from_slice(&payload_len.to_le_bytes());
    f.extend_from_slice(body);
    f
}

pub fn le16(b: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([b[off], b[off + 1]])
}

pub fn le32(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}
