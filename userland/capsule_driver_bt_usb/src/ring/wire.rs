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

//! The keyring's record operations, laid out as `capsule_keyring` reads
//! them (`server/handlers/store.rs`, `retrieve.rs` and `delete.rs`), and
//! OP_LIST as lead unit L33 gives it. Each request names the caller's pid,
//! which the keyring holds against the pid the kernel stamps on the message,
//! so one capsule cannot reach another's records.

use alloc::vec::Vec;

pub const OP_STORE: u16 = 1;
pub const OP_RETRIEVE: u16 = 2;
pub const OP_DELETE: u16 = 3;
pub const OP_LIST: u16 = 31;

/// No end time: a bond goes when it is deleted, or with the boot.
const NEVER: u64 = 0;

/// OP_STORE: pid, now, expiry, key type, data length, data. None when the
/// data is longer than its 16-bit length can say.
pub fn store(pid: u32, now_ms: u64, kind: u8, data: &[u8]) -> Option<Vec<u8>> {
    let len = u16::try_from(data.len()).ok()?;
    let mut p = Vec::with_capacity(23 + data.len());
    p.extend_from_slice(&pid.to_le_bytes());
    p.extend_from_slice(&now_ms.to_le_bytes());
    p.extend_from_slice(&NEVER.to_le_bytes());
    p.push(kind);
    p.extend_from_slice(&len.to_le_bytes());
    p.extend_from_slice(data);
    Some(p)
}

/// OP_RETRIEVE and OP_DELETE: pid, then the record's id.
pub fn by_id(pid: u32, id: u32) -> [u8; 8] {
    let ([a, b, c, d], [e, f, g, h]) = (pid.to_le_bytes(), id.to_le_bytes());
    [a, b, c, d, e, f, g, h]
}

/// OP_LIST: pid, then the key type.
pub fn list(pid: u32, kind: u8) -> [u8; 5] {
    let [a, b, c, d] = pid.to_le_bytes();
    [a, b, c, d, kind]
}
