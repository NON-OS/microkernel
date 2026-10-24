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

//! The one request the shell sends to power off, and what an answer to it
//! means. Little-endian, in the 20-byte header the system services use:
//! magic, version, op, flags, a reserved half-word, the request id and the
//! payload length. The request has no payload. A power off that works never
//! answers, because the machine is gone; any answer is a refusal, its status
//! an i32 after the header.

/// The endpoint the kernel holds to the shell (its registry's held table).
pub const SERVICE: &[u8] = b"power.off";
/// "POFF".
pub const MAGIC: u32 = 0x4646_4F50;
pub const VERSION: u16 = 1;
pub const OP_OFF: u16 = 0x0001;
pub const HDR_LEN: usize = 20;
const STATUS_LEN: usize = 4;

pub fn request(request_id: u32) -> [u8; HDR_LEN] {
    let mut f = [0u8; HDR_LEN];
    f[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    f[4..6].copy_from_slice(&VERSION.to_le_bytes());
    f[6..8].copy_from_slice(&OP_OFF.to_le_bytes());
    f[12..16].copy_from_slice(&request_id.to_le_bytes());
    f
}

/// What came back from the call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Heard {
    /// The service answered with this status: it did not power off.
    Refused(i32),
    /// An answer too short, or not to this request: still not powered off.
    Garbled,
    /// No answer in the time allowed, or the call failed.
    Silent,
}

/// Read the call's result `rc` (the reply length, or a negative error) and
/// the reply buffer, against the request id that was sent.
pub fn heard(rc: i64, reply: &[u8], request_id: u32) -> Heard {
    let Ok(n) = usize::try_from(rc) else { return Heard::Silent };
    let Some(r) = reply.get(..n).filter(|r| r.len() >= HDR_LEN + STATUS_LEN) else {
        return Heard::Garbled;
    };
    let word = |at: usize| u32::from_le_bytes([r[at], r[at + 1], r[at + 2], r[at + 3]]);
    if word(0) != MAGIC || word(12) != request_id {
        return Heard::Garbled;
    }
    Heard::Refused(word(HDR_LEN) as i32)
}
