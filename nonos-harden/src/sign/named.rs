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

//! What a signed verdict says of itself: the signer it names, and its one
//! line; and the public key a verifier gives, in hex or as a NONOS key file.

use nonos_capsule_sign::{algs::AlgId, keys::read_pub};
use std::path::Path;

/// 64 hex digits, or a NONOS public key file (NONOSPK1) holding an Ed25519 key.
pub fn public_key(given: &str) -> Result<[u8; 32], String> {
    let mut out = [0u8; 32];
    if hex::decode_to_slice(given, &mut out).is_ok() {
        return Ok(out);
    }
    let k = read_pub(Path::new(given)).map_err(|e| format!("{given}: {e}"))?;
    let ed = (k.alg == AlgId::Ed25519).then(|| <[u8; 32]>::try_from(k.bytes.as_slice()).ok());
    ed.flatten().ok_or(format!("{given} holds no Ed25519 public key"))
}

pub fn signer_named(text: &str) -> Option<String> {
    match serde_json::from_str::<serde_json::Value>(text) {
        Ok(v) => v.get("signer").and_then(|s| s.as_str()).map(String::from),
        Err(_) => text.lines().find_map(|l| l.strip_prefix("signer: ")).map(String::from),
    }
}

pub fn line(text: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(text) {
        Ok(v) => v["verdict"].as_str().unwrap_or_default().to_string(),
        // The one line is the last not indented; the limits follow it.
        Err(_) => text.lines().rfind(|l| !l.starts_with(' ')).unwrap_or_default().into(),
    }
}
