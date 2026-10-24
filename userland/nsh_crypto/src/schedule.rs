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

//! The key schedule (nsh-v1-wire.md, "Keys"). The handshake secret is
//! extracted from the X-Wing secret under `TH(SERVER_HELLO)`; the session
//! secret from the handshake secret under `TH(AUTH_OK)`. Each direction
//! has its own key, so a frame sent one way can never be opened as one
//! sent the other way. Taking the session keys consumes the handshake
//! secret, which is then wiped.

use crate::Secret;
use nonos_hash::{hkdf512_expand, hkdf512_extract};

pub struct Keys {
    pub c2s: Secret<32>,
    pub s2c: Secret<32>,
}

fn expand(prk: &Secret<64>, label: &[u8]) -> Secret<32> {
    let mut out = [0u8; 32];
    // 32 bytes is one block, far inside HKDF's limit.
    let _ = hkdf512_expand(prk.expose(), label, &mut out);
    Secret::take(&mut out)
}

/// The handshake keys, and the handshake secret the session keys come from.
pub fn handshake_keys(ss: Secret<32>, th_server_hello: &[u8; 64]) -> (Keys, Secret<64>) {
    let hs = Secret::take(&mut hkdf512_extract(th_server_hello, ss.expose()));
    let keys = Keys { c2s: expand(&hs, b"nsh/1 c2s hs"), s2c: expand(&hs, b"nsh/1 s2c hs") };
    (keys, hs)
}

pub fn session_keys(hs: Secret<64>, th_auth_ok: &[u8; 64]) -> Keys {
    let ts = Secret::take(&mut hkdf512_extract(th_auth_ok, hs.expose()));
    Keys { c2s: expand(&ts, b"nsh/1 c2s data"), s2c: expand(&ts, b"nsh/1 s2c data") }
}
