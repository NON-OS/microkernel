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

//! The two byte strings ntor hashes, assembled from parts and nothing else.

use super::constants::{AUTH_INPUT_BYTES, IDENTITY_BYTES, PROTOID, SECRET_INPUT_BYTES, SERVER};

pub struct Parts<'a> {
    /// EXP(Y,x)
    pub xy: &'a [u8; 32],
    /// EXP(B,x)
    pub xb: &'a [u8; 32],
    pub identity: &'a [u8; IDENTITY_BYTES],
    /// B
    pub onion_key: &'a [u8; 32],
    /// X
    pub client: &'a [u8; 32],
    /// Y
    pub server: &'a [u8; 32],
}

/// EXP(Y,x) || EXP(B,x) || ID || B || X || Y || PROTOID
///
pub fn secret_input(parts: &Parts<'_>) -> [u8; SECRET_INPUT_BYTES] {
    let mut out = [0u8; SECRET_INPUT_BYTES];
    let fields: [&[u8]; 7] =
        [parts.xy, parts.xb, parts.identity, parts.onion_key, parts.client, parts.server, PROTOID];
    write(&mut out, &fields);
    out
}

/// verify || ID || B || Y || X || PROTOID || "Server"
pub fn auth_input(parts: &Parts<'_>, verify: &[u8; 32]) -> [u8; AUTH_INPUT_BYTES] {
    let mut out = [0u8; AUTH_INPUT_BYTES];
    let fields: [&[u8]; 7] =
        [verify, parts.identity, parts.onion_key, parts.server, parts.client, PROTOID, SERVER];
    write(&mut out, &fields);
    out
}

fn write(out: &mut [u8], fields: &[&[u8]]) {
    let mut at = 0usize;
    for field in fields {
        let end = at + field.len();
        if end > out.len() {
            return;
        }
        out[at..end].copy_from_slice(field);
        at = end;
    }
}
