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

//! The 20-octet header every request and reply carries. The frame is NNET's
//! (nonos_usbnet/src/nnet/wire.rs): magic, version, op, flags, a zero
//! half-word, the request id and the payload's length. A reply's payload is
//! a status word and then the op's own payload.

/// "NBTC", a namespace of its own, so a frame meant for another service is
/// never read as one of these.
pub const MAGIC: u32 = u32::from_le_bytes(*b"NBTC");
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;
pub const STATUS_LEN: usize = 4;
/// Where a reply's own payload starts.
pub const DATA_AT: usize = HDR_LEN + STATUS_LEN;

/// A request for `op` with `body` into `out`: its length, or `None` when it
/// does not fit.
pub fn request(out: &mut [u8], op: u16, request_id: u32, body: &[u8]) -> Option<usize> {
    let n = HDR_LEN.checked_add(body.len())?;
    let len = u32::try_from(body.len()).ok()?;
    let out = out.get_mut(..n)?;
    out[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&op.to_le_bytes());
    // Flags and the reserved half-word: v1 sets neither.
    out[8..12].fill(0);
    out[12..16].copy_from_slice(&request_id.to_le_bytes());
    out[16..20].copy_from_slice(&len.to_le_bytes());
    out[HDR_LEN..].copy_from_slice(body);
    Some(n)
}

/// The status and payload of the reply to `op` and `request_id`. `None` for
/// anything else: another service's frame, a reply to another request, or
/// one whose length is not what came.
pub fn read_reply(buf: &[u8], op: u16, request_id: u32) -> Option<(i32, &[u8])> {
    if buf.len() < DATA_AT {
        return None;
    }
    let word = |at: usize| u32::from_le_bytes([buf[at], buf[at + 1], buf[at + 2], buf[at + 3]]);
    let half = |at: usize| u16::from_le_bytes([buf[at], buf[at + 1]]);
    let fits = word(16) as usize == buf.len() - HDR_LEN;
    let ours = word(0) == MAGIC && half(4) == VERSION && half(6) == op && half(10) == 0;
    if !fits || !ours || word(12) != request_id {
        return None;
    }
    Some((word(HDR_LEN) as i32, &buf[DATA_AT..]))
}
