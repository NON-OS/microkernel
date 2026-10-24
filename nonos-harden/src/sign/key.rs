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

//! The operator's Ed25519 key: the raw 32-byte seed, a NONOS seed file
//! (NONOSSK1, nonos-sign's format), or the seed as 64 hex digits. It is
//! refused by where it lies before it is read, and by its public key once
//! it is, and the bytes read are cleared as soon as the key is made.

use super::refuse::{by_place, by_public};
use crate::release::read::read_regular;
use ed25519_dalek::SigningKey;
use nonos_capsule_sign::algs::AlgId;
use std::path::Path;

pub fn load(path: &Path, tree: Option<&Path>) -> Result<SigningKey, String> {
    by_place(path)?;
    let mut bytes = read_regular(path, 4096).map_err(|e| format!("{}: {e}", path.display()))?;
    let seed = seed(&bytes, path);
    bytes.fill(0);
    let key = SigningKey::from_bytes(&seed?);
    if let Some(tree) = tree {
        by_public(tree, key.verifying_key().as_bytes())?;
    }
    Ok(key)
}

pub(crate) fn seed(bytes: &[u8], path: &Path) -> Result<[u8; 32], String> {
    if let Ok(raw) = <[u8; 32]>::try_from(bytes) {
        return Ok(raw);
    }
    // nonos-sign's seed file (keys/read.rs), read from the bytes in hand so
    // they are the bytes cleared: magic, algorithm, length (big-endian), seed.
    if let Some(rest) = bytes.strip_prefix(b"NONOSSK1") {
        let ed = rest.first().is_some_and(|&a| AlgId::from_u8(a).ok() == Some(AlgId::Ed25519));
        return match (ed, rest.get(1..3), rest.get(3..)) {
            (true, Some([0, 32]), Some(k)) if k.len() == 32 => Ok(k.try_into().unwrap_or([0; 32])),
            (false, _, _) => Err("a NONOS seed file, but not an Ed25519 one".into()),
            _ => Err(format!("{}: not a well-formed NONOS seed file", path.display())),
        };
    }
    let mut out = [0u8; 32];
    let text = std::str::from_utf8(bytes).map(str::trim).unwrap_or_default();
    match hex::decode_to_slice(text, &mut out) {
        Ok(()) => Ok(out),
        Err(_) => {
            Err("not an Ed25519 key: give its 32-byte seed, 64 hex digits, or a NONOS seed file"
                .into())
        }
    }
}
