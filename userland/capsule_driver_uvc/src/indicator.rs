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
//! The compositor's capture indicator, pure: the notice sent whenever the
//! camera starts or stops (NCMP version 1, op 000Ah, body `kind u32` (1,
//! the camera) and `on u32`), and whether its reply says the indicator is
//! shown. Under Decision 9 no indicator means no stream, so anything but a
//! whole NCMP reply with status 0 is a refusal.

const MAGIC: u32 = 0x4E43_4D50; // "NCMP"
const VERSION: u16 = 1;
const OP_CAPTURE: u16 = 0x000A;
const CAMERA: u32 = 1;
/// The reply's status word, after the 20-byte header.
const STATUS_AT: usize = 20;

pub fn notice(on: bool) -> [u8; 28] {
    let mut req = [0u8; 28];
    req[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    req[4..6].copy_from_slice(&VERSION.to_le_bytes());
    req[6..8].copy_from_slice(&OP_CAPTURE.to_le_bytes());
    req[16..20].copy_from_slice(&8u32.to_le_bytes());
    req[20..24].copy_from_slice(&CAMERA.to_le_bytes());
    req[24..28].copy_from_slice(&(on as u32).to_le_bytes());
    req
}

/// Whether a call that returned `rc` with `resp` shows the indicator.
pub fn shown(rc: i64, resp: &[u8]) -> bool {
    let at = |i: usize| resp.get(i..i + 4).map(|b| [b[0], b[1], b[2], b[3]]);
    let whole = rc >= (STATUS_AT + 4) as i64;
    let ncmp = at(0).map(u32::from_le_bytes) == Some(MAGIC)
        && resp.get(6..8) == Some(&OP_CAPTURE.to_le_bytes()[..]);
    whole && ncmp && at(STATUS_AT).map(i32::from_le_bytes) == Some(0)
}
