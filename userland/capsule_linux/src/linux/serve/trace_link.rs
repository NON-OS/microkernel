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

//! One link of the diagnostic chain (plan/32): capsule_linux's own label for
//! a call it answered, folded into the head before it.
//!
//!   d' = BLAKE3("NONOS-LINUX-DIAG-v1" | d | nr LE64 | label u8 | errno LE16)
//!
//! from d = 32 zero bytes. The label is SERVED, FAILED with its errno, or
//! UNSERVED (ENOSYS from a call this capsule has no answer for). The head is
//! private: it goes only into the sealed witness, never the record.

pub const DOMAIN: &[u8] = b"NONOS-LINUX-DIAG-v1";
pub const SERVED: u8 = 0;
pub const FAILED: u8 = 1;
pub const UNSERVED: u8 = 2;

/// The label and errno of a call that returned `result`.
pub fn label(result: u64, unserved: bool) -> (u8, u16) {
    let value = result as i64;
    match (unserved, value) {
        (true, _) => (UNSERVED, 38),
        (false, -4095..=-1) => (FAILED, (-value) as u16),
        _ => (SERVED, 0),
    }
}

pub fn link(head: &[u8; 32], nr: u64, label: u8, errno: u16) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(DOMAIN);
    h.update(head);
    h.update(&nr.to_le_bytes());
    h.update(&[label]);
    h.update(&errno.to_le_bytes());
    *h.finalize().as_bytes()
}
