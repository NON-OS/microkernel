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

//! A list as a whole: its canonical form, its entries and its hash.

use super::entry::{read, Dest};
use super::error::DestsError;

/// The domain net_dests_hash is taken under (plan/30).
pub const DOMAIN: &[u8] = b"NONOS.linux.netdests.v1";
/// The most entries one list holds.
pub const MAX_ENTRIES: usize = 64;

/// How many entries `list` holds, when it is canonical.
pub fn check(list: &[u8]) -> Result<usize, DestsError> {
    let (count, mut rest) = match list {
        [hi, lo, rest @ ..] => (usize::from(u16::from_be_bytes([*hi, *lo])), rest),
        _ => return Err(DestsError::Short),
    };
    if count == 0 || count > MAX_ENTRIES {
        return Err(DestsError::Count);
    }
    let mut before: &[u8] = &[];
    for _ in 0..count {
        let (_, len) = read(rest)?;
        let (this, after) = rest.split_at(len);
        if this <= before {
            return Err(DestsError::Order);
        }
        (before, rest) = (this, after);
    }
    match rest.is_empty() {
        true => Ok(count),
        false => Err(DestsError::Trailing),
    }
}

/// BLAKE3("NONOS.linux.netdests.v1" | list): the policy's net_dests_hash.
pub fn dests_hash(list: &[u8]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(DOMAIN);
    h.update(list);
    *h.finalize().as_bytes()
}

/// The entries of `list`, which `check` has accepted. Reading stops at the
/// first byte that is not an entry, so an unchecked list yields a prefix.
pub fn entries(list: &[u8]) -> impl Iterator<Item = Dest<'_>> {
    let mut rest = list.get(2..).unwrap_or_default();
    core::iter::from_fn(move || {
        let (dest, len) = read(rest).ok()?;
        rest = &rest[len..];
        Some(dest)
    })
}
