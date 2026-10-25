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

//! The MADT's structures from the table's bytes, with no pointer read:
//! after the 36-byte header, the local APIC address u32 and flags u32,
//! each structure is a type u8 and a length u8. The declared length is
//! believed only as far as the bytes handed over; a structure shorter than
//! its own two bytes or running past the table ends the walk, and past
//! `MAX_STRUCTURES` the rest are not read. `kernel_proofs` mounts this file.

pub const STRUCTURES_AT: usize = 44;
/// Two per processor (its APIC and its NMI line) on the largest machines
/// the x2APIC entries describe, with room over.
pub const MAX_STRUCTURES: usize = 8192;

pub fn structures(t: &[u8]) -> impl Iterator<Item = &[u8]> {
    let declared = t.get(4..8).map_or(0, |h| u32::from_le_bytes([h[0], h[1], h[2], h[3]]));
    let t = &t[..(declared as usize).min(t.len())];
    let mut at = STRUCTURES_AT;
    let next = move || {
        let len = usize::from(*t.get(at + 1)?);
        let s = t.get(at..at + len).filter(|_| len >= 2)?;
        at += len;
        Some(s)
    };
    core::iter::from_fn(next).take(MAX_STRUCTURES)
}
