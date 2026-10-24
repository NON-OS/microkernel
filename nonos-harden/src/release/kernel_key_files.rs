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

//! The kernel's two public key files, read in the formats the seal writes.

use super::kernel_keys::ED_PUB;
use super::read::read_regular;
use nonos_capsule_sign::algs::AlgId;
use nonos_capsule_sign::keys::read_pub;
use std::path::Path;

// The Ed25519 key as the seal writes it (tools/nonos_seal/keys.py) and the
// loader's build reads it (its last 32 bytes, tools/nix/image.nix): the raw
// 32-byte public key, no header. Anything else is not the key the loader holds.
pub(super) fn ed_key(tree: &Path) -> Result<Vec<u8>, String> {
    let raw = read_regular(&tree.join(ED_PUB), 4096).map_err(|e| format!("{ED_PUB}: {e}"))?;
    match raw.len() {
        32 => Ok(raw),
        n => Err(format!("{ED_PUB} holds {n} bytes, not the raw 32-byte key the seal writes")),
    }
}

pub(super) fn key(tree: &Path, rel: &str, alg: AlgId) -> Result<Vec<u8>, String> {
    let k = read_pub(&tree.join(rel)).map_err(|e| format!("{rel}: {e}"))?;
    match k.alg == alg {
        true => Ok(k.bytes.clone()),
        false => Err(format!("{rel} is not a {} key", alg.label())),
    }
}
