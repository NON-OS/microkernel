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

//! A transferable public key, as `gpg --export` writes it: every key and
//! subkey packet in it. The whole file is what is pinned, so a subkey is
//! trusted because it is in the file, not because of a binding signature
//! read here.

use alloc::vec::Vec;

use super::key::{key, Key};
use super::packet::next;
use super::subpacket::Found;

const PUBLIC_KEY: u8 = 6;
const PUBLIC_SUBKEY: u8 = 14;

/// None if the file does not frame, or holds no key this crate can use.
pub fn keys(ring: &[u8]) -> Option<Vec<Key>> {
    let mut out = Vec::new();
    let mut rest = ring;
    while !rest.is_empty() {
        let (p, after) = next(rest)?;
        if p.tag == PUBLIC_KEY || p.tag == PUBLIC_SUBKEY {
            out.extend(key(p.body));
        }
        rest = after;
    }
    (!out.is_empty()).then_some(out)
}

/// The key a signature names: by fingerprint when it gives one, else by the
/// low eight bytes of it. Naming no key finds none.
pub fn issuer_key<'a>(ring: &'a [Key], found: &Found) -> Option<&'a Key> {
    ring.iter().find(|k| match (found.fingerprint, found.key_id) {
        (Some(f), _) => k.fingerprint == f,
        (None, Some(id)) => k.fingerprint[12..] == id,
        (None, None) => false,
    })
}
