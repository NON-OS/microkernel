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

//! What an image install placed: the image's pin, and each pack it landed
//! with the BLAKE3 of that pack's header and index, the bytes this machine
//! minted the pack's trailer over. The install keeps it, minted by this
//! machine too (reuse.rs), so a later install of the same pin can see every
//! pack is still the one placed and register it without fetching, decoding
//! or placing anything. Pure, so the host proofs hold it.
//!
//! `"NONOSPL1" | pin (32) | count (u16) | (name length (u8) | name | digest (32))...`

use alloc::string::String;
use alloc::vec::Vec;

pub const MAGIC: [u8; 8] = *b"NONOSPL1";
/// The most packs one record names: an image's members (nonos_app_image).
pub const MAX: usize = nonos_app_image::MAX_MEMBERS;
/// A record at its longest: every name at the store's longest pack name.
pub const MAX_BYTES: usize = 8 + 32 + 2 + MAX * (1 + 255 + 32);

/// One pack placed: its name and its head's digest.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Placed {
    pub name: String,
    pub head: [u8; 32],
}

/// The record for `pin`, or None when it would not read back.
pub fn encode(pin: &[u8; 32], packs: &[Placed]) -> Option<Vec<u8>> {
    if packs.is_empty() || packs.len() > MAX {
        return None;
    }
    let mut out = Vec::with_capacity(42 + packs.len() * 64);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(pin);
    out.extend_from_slice(&(packs.len() as u16).to_le_bytes());
    for p in packs {
        let n = p.name.as_bytes();
        if n.is_empty() || n.len() > 255 || n.contains(&b'/') {
            return None;
        }
        out.push(n.len() as u8);
        out.extend_from_slice(n);
        out.extend_from_slice(&p.head);
    }
    Some(out)
}

/// The packs a record for `pin` names; None for any other pin, any
/// malformed record, or one with bytes after its last pack.
pub fn decode(raw: &[u8], pin: &[u8; 32]) -> Option<Vec<Placed>> {
    if raw.len() < 42 || raw.len() > MAX_BYTES || raw[..8] != MAGIC || raw[8..40] != pin[..] {
        return None;
    }
    let count = usize::from(u16::from_le_bytes([raw[40], raw[41]]));
    if count == 0 || count > MAX {
        return None;
    }
    let (mut at, mut out) = (42usize, Vec::with_capacity(count));
    for _ in 0..count {
        let len = usize::from(*raw.get(at)?);
        let name = raw.get(at + 1..at + 1 + len)?;
        let head = raw.get(at + 1 + len..at + 1 + len + 32)?;
        if len == 0 || name.contains(&b'/') {
            return None;
        }
        let mut digest = [0u8; 32];
        digest.copy_from_slice(head);
        out.push(Placed { name: String::from(core::str::from_utf8(name).ok()?), head: digest });
        at += 1 + len + 32;
    }
    (at == raw.len()).then_some(out)
}
