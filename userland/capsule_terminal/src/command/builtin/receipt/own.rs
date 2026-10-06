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

//! What signed this terminal, as the kernel recorded it when it admitted the
//! capsule. `whoami`, `version` and `neofetch` used to print a fixed line
//! ("signed by NONOS publisher cert (Ed25519+ML-DSA-65)") that no check stood
//! behind; they now read this terminal's own entry in the attestation
//! registry, the same entries `receipt` lists.

use alloc::vec::Vec;

/// Bytes of a registry entry: pid, measurement, capability mask, authority.
pub const ENTRY_LEN: usize = 45;
const MEASURE_AT: usize = 4;
const AUTHORITY_AT: usize = 44;
/// Measurement bytes shown, as in `receipt`'s column.
const MEASURE_SHOWN: usize = 6;

/// The entry for `pid` among the registry's `entries`, if it lists one.
pub fn entry_for(entries: &[u8], pid: u32) -> Option<&[u8]> {
    entries.chunks_exact(ENTRY_LEN).find(|e| e[..4] == pid.to_be_bytes())
}

/// Who signed a capsule, from the kernel's authority byte: 0 the vendor, 255
/// the publisher, anything else a developer key enrolled on this machine.
pub fn signer(authority: u8) -> &'static [u8] {
    match authority {
        0 => b"vendor",
        255 => b"publisher",
        _ => b"developer",
    }
}

/// "publisher, measurement 1a2b3c4d5e6f" for an entry.
pub fn describe(entry: &[u8]) -> Vec<u8> {
    let mut out = Vec::from(signer(entry[AUTHORITY_AT]));
    out.extend_from_slice(b", measurement ");
    for b in &entry[MEASURE_AT..MEASURE_AT + MEASURE_SHOWN] {
        for nib in [b >> 4, b & 0xf] {
            out.push(if nib < 10 { b'0' + nib } else { b'a' + nib - 10 });
        }
    }
    out
}

/// What to print for this terminal given the registry read: the entry, or why
/// there is none. `read` is the bytes the kernel returned, or its errno.
pub fn own_signer(read: Result<&[u8], i64>, pid: u32) -> Vec<u8> {
    match read {
        Err(_) => Vec::from(&b"unknown: the kernel did not hand over its registry"[..]),
        Ok(entries) => match entry_for(entries, pid) {
            Some(e) => describe(e),
            None => Vec::from(&b"unknown: this terminal is not in the kernel's registry"[..]),
        },
    }
}
