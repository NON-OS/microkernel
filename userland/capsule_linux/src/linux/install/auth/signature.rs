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

//! The signature an index carries in its first member.

use alloc::vec::Vec;

use nonos_inflate::members;

use super::super::tar::{entries, Entry};
use super::digest::{sha1, sha256};
use super::keys::spki;

/// Asks whether `sig` over `digest` verifies under `spki`; `hashid` 3 is
/// SHA-1 and 0 is SHA-256, as the crypto service numbers them.
pub type Rsa = dyn Fn(&[u8], &[u8], u8, &[u8]) -> bool;

/// The index tar, if a key trusted here signed the member that holds it.
/// An index is exactly two members, so nothing unsigned rides along.
pub fn signed_index(raw: &[u8], rsa: &Rsa) -> Option<Vec<u8>> {
    let mut parts = members(raw)?;
    if parts.len() != 2 {
        return None;
    }
    let index = parts.pop()?;
    let covered = raw.get(index.start..index.end)?;
    signed(&parts[0].body, covered, rsa).then_some(index.body)
}

/// True when some signature entry in `sig_tar` verifies over `covered`.
pub fn signed(sig_tar: &[u8], covered: &[u8], rsa: &Rsa) -> bool {
    entries(sig_tar).iter().any(|entry| one(entry, covered, rsa))
}

fn one(entry: &Entry, covered: &[u8], rsa: &Rsa) -> bool {
    let (hashid, key) = if let Some(k) = entry.name.strip_prefix(b".SIGN.RSA256.") {
        (0u8, k)
    } else if let Some(k) = entry.name.strip_prefix(b".SIGN.RSA.") {
        (3u8, k)
    } else {
        return false;
    };
    let Some(key) = spki(key) else {
        return false;
    };
    match hashid {
        0 => rsa(&key, &entry.body, hashid, &sha256(covered)),
        _ => rsa(&key, &entry.body, hashid, &sha1(covered)),
    }
}
