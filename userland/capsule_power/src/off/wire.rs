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

//! The shell's power-off request on `power.off`, as G-53 sends it: one
//! 20-byte header, little-endian, no payload. Magic u32 "POFF", version
//! u16 1, op u16 1 (OFF), flags u16 0, reserved u16 0, request id u32,
//! payload length u32 0. A refusal echoes the header with an i32 status at
//! offset 20; a request that runs is never answered. Pure.

pub const MAGIC: u32 = u32::from_le_bytes(*b"POFF");
pub const VERSION: u16 = 1;
pub const OP_OFF: u16 = 1;
pub const HEADER: usize = 20;

/// Whether `req` is meant as a power-off request: its first four bytes are
/// the magic. Its first byte, `P`, is no op of the `acpi` port.
pub fn is_poff(req: &[u8]) -> bool {
    req.get(..4) == Some(&MAGIC.to_le_bytes()[..])
}

fn u16_at(r: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([r[at], r[at + 1]])
}

/// The request exactly as the shell sends it, and nothing else.
pub fn well_formed(req: &[u8]) -> bool {
    req.len() == HEADER
        && is_poff(req)
        && u16_at(req, 4) == VERSION
        && u16_at(req, 6) == OP_OFF
        && u16_at(req, 8) == 0
        && u16_at(req, 10) == 0
        && req[16..20] == [0; 4]
}

/// The refusal: the header as it came (cut or zero-padded to 20 bytes),
/// then the negative status.
pub fn refusal(req: &[u8], status: i32) -> [u8; HEADER + 4] {
    let mut out = [0u8; HEADER + 4];
    let n = req.len().min(HEADER);
    out[..n].copy_from_slice(&req[..n]);
    out[HEADER..].copy_from_slice(&status.to_le_bytes());
    out
}
