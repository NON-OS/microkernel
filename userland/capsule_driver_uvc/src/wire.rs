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
//! The camera wire, NUVC version 1: the host wire's 20-byte header with its
//! own magic (op u16 at 6, request id u32 at 12, payload length u32 at
//! 16), then the body; a reply's payload starts with its status.

pub const MAGIC: u32 = 0x4E55_5643; // "NUVC"
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;

pub const OP_HEALTHCHECK: u16 = 1;
/// The camera's formats and frame sizes (listing.rs).
pub const OP_OPEN: u16 = 2;
/// `format u8, frame u8, 0, 0, interval u32` (100 ns units, 0 for the
/// frame's default): the most bytes a frame takes, as u32.
pub const OP_START: u16 = 3;
/// `offset u32`: `sequence u32, total u32, offset u32`, then up to
/// CHUNK bytes of the last whole frame from `offset`.
pub const OP_READ_FRAME: u16 = 4;
pub const OP_STOP: u16 = 5;

/// A frame is read in pieces this size until a shared-memory path exists.
pub const CHUNK: usize = 4000;
pub const REPLY_MAX: usize = HDR_LEN + 4 + 12 + CHUNK;

/// The caller lacks CameraCapture.
pub const E_PERM: i32 = -1;
pub const E_AGAIN: i32 = -11;
pub const E_BUSY: i32 = -16;
pub const E_NODEV: i32 = -19;
pub const E_INVAL: i32 = -22;

pub struct Request {
    pub op: u16,
    pub id: u32,
}

/// The request in `msg` and its body, or None when it is not one.
pub fn decode(msg: &[u8]) -> Option<(Request, &[u8])> {
    let at = |i: usize| msg.get(i..i + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    if at(0)? != MAGIC || msg.get(4..6)? != VERSION.to_le_bytes() {
        return None;
    }
    let op = u16::from_le_bytes([msg[6], msg[7]]);
    let body = msg.get(HDR_LEN..HDR_LEN + at(16)? as usize)?;
    Some((Request { op, id: at(12)? }, body))
}

/// A reply of `status` and `data` in `tx`; its length.
pub fn reply(tx: &mut [u8], req: &Request, status: i32, data: &[u8]) -> usize {
    let n = data.len().min(tx.len() - HDR_LEN - 4);
    tx[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    tx[4..6].copy_from_slice(&VERSION.to_le_bytes());
    tx[6..8].copy_from_slice(&req.op.to_le_bytes());
    tx[8..12].fill(0);
    tx[12..16].copy_from_slice(&req.id.to_le_bytes());
    tx[16..20].copy_from_slice(&(4 + n as u32).to_le_bytes());
    tx[20..24].copy_from_slice(&status.to_le_bytes());
    tx[24..24 + n].copy_from_slice(&data[..n]);
    HDR_LEN + 4 + n
}
