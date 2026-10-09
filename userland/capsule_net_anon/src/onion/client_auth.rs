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


//! Client authorization: reaching a service that encrypts its descriptor's
//! inner layer to a list of clients (rend-spec-v3 section 2.5.1.2, the
//! fork's build_descriptor_cookie_keys and decrypt_descriptor_cookie).
//!
//! The service puts one `auth-client` line per authorized client in the
//! middle layer, each holding the descriptor cookie encrypted to that
//! client. A client that holds an X25519 secret for the service finds its
//! line by client id and recovers the cookie; the inner layer is keyed with
//! the blinded key and the cookie together. The secret is held in memory
//! only, wiped when dropped, and never named on the log.

extern crate alloc;

use alloc::vec::Vec;

use nonos_aes::Ctr256Be;

use crate::crypto::keccak::Shake256;
use crate::directory::base64::decode;
use crate::directory::lines::{arg, lines};

use super::address::parse;

pub const COOKIE_BYTES: usize = 16;
/// A service adds fake entries to reach a multiple of sixteen; anything far
/// past that is not a descriptor anybody would publish.
const AUTH_CLIENTS_MAX: usize = 1024;

/// One client's key for one service, from a line in the fork's
/// `.auth_private` format: `<address>:descriptor:x25519:<base32 secret>`.
pub struct ClientKey {
    pub identity: [u8; 32],
    secret: [u8; 32],
}

impl ClientKey {
    pub fn secret(&self) -> &[u8; 32] {
        &self.secret
    }
}

impl Drop for ClientKey {
    fn drop(&mut self) {
        wipe(&mut self.secret);
    }
}

/// Parse one key line. The address may carry the `.anyone` suffix or not.
/// `None` for any other shape, an auth or key type other than
/// descriptor/x25519, a secret that is not 32 bytes, or an all zero one.
pub fn parse_key(line: &[u8]) -> Option<ClientKey> {
    let line = line.strip_suffix(b"\n").unwrap_or(line);
    let line = line.strip_suffix(b"\r").unwrap_or(line);
    let fields: Vec<&[u8]> = line.split(|b| *b == b':').map(|f| f.trim_ascii()).collect();
    let [address, auth, kind, secret] = fields.as_slice() else { return None };
    if *auth != b"descriptor" || *kind != b"x25519" {
        return None;
    }
    let identity = if crate::onion::address::is_onion(address) {
        parse(address)?
    } else {
        let mut host = address.to_vec();
        host.extend_from_slice(crate::onion::address::SUFFIX);
        parse(&host)?
    };
    let secret = base32_key(secret)?;
    if secret.iter().all(|b| *b == 0) {
        return None;
    }
    Some(ClientKey { identity, secret })
}

/// The middle layer's client list: the service's ephemeral key and every
/// `auth-client` entry (client id, IV, encrypted cookie).
pub struct AuthLayer {
    pub ephemeral: [u8; 32],
    pub clients: Vec<([u8; 8], [u8; 16], [u8; COOKIE_BYTES])>,
}

/// Read the client list out of the middle layer. `None` when it has no
/// `desc-auth-type x25519` or no well formed ephemeral key.
pub fn auth_layer(middle: &[u8]) -> Option<AuthLayer> {
    let mut x25519 = false;
    let mut ephemeral = None;
    let mut clients = Vec::new();
    for line in lines(middle) {
        match line.keyword {
            b"desc-auth-type" => x25519 = arg(line.rest, 0) == Some(b"x25519"),
            b"desc-auth-ephemeral-key" => ephemeral = arg(line.rest, 0).and_then(decode).and_then(|k| k.try_into().ok()),
            b"auth-client" if clients.len() < AUTH_CLIENTS_MAX => {
                let field = |i| arg(line.rest, i).and_then(decode);
                if let (Some(id), Some(iv), Some(cookie)) = (field(0), field(1), field(2)) {
                    if let (Ok(id), Ok(iv), Ok(cookie)) = (id.try_into(), iv.try_into(), cookie.try_into()) {
                        clients.push((id, iv, cookie));
                    }
                }
            }
            _ => {}
        }
    }
    if !x25519 {
        return None;
    }
    Some(AuthLayer { ephemeral: ephemeral?, clients })
}

/// The descriptor cookie, given `seed` = X25519(client secret, the layer's
/// ephemeral key). Every entry is compared, in constant time, so the time
/// taken does not say which line is ours. `None` when no entry is.
pub fn cookie(subcredential: &[u8; 32], seed: &[u8; 32], layer: &AuthLayer) -> Option<[u8; COOKIE_BYTES]> {
    let mut kdf = Shake256::new();
    kdf.update(subcredential);
    kdf.update(seed);
    let mut client_id = [0u8; 8];
    let mut cookie_key = [0u8; 32];
    kdf.squeeze(&mut client_id);
    kdf.squeeze(&mut cookie_key);
    let mut found: Option<([u8; 16], [u8; COOKIE_BYTES])> = None;
    for (id, iv, encrypted) in layer.clients.iter() {
        if crate::crypto::equal(id, &client_id) && found.is_none() {
            found = Some((*iv, *encrypted));
        }
    }
    let result = found.map(|(iv, encrypted)| {
        let mut out = encrypted;
        Ctr256Be::with_iv(&cookie_key, &iv).apply(&mut out);
        out
    });
    wipe(&mut cookie_key);
    result
}

/// 52 base32 characters, no padding: 260 bits, of which the last four must
/// be zero, so no two strings decode to one key.
fn base32_key(text: &[u8]) -> Option<[u8; 32]> {
    if text.len() != 52 {
        return None;
    }
    let mut out = [0u8; 32];
    let (mut acc, mut bits, mut at) = (0u32, 0u32, 0usize);
    for c in text {
        let value = match c {
            b'a'..=b'z' => c - b'a',
            b'A'..=b'Z' => c - b'A',
            b'2'..=b'7' => c - b'2' + 26,
            _ => return None,
        } as u32;
        acc = (acc << 5) | value;
        bits += 5;
        if bits >= 8 {
            bits -= 8;
            out[at] = (acc >> bits) as u8;
            at += 1;
        }
    }
    if acc & ((1 << bits) - 1) != 0 {
        return None;
    }
    Some(out)
}

pub(crate) fn wipe(bytes: &mut [u8]) {
    for byte in bytes.iter_mut() {
        /* SAFETY: eK@nonos.systems. Client secrets and cookie keys, wiped
         * with a store the optimiser may not remove. */
        unsafe { core::ptr::write_volatile(byte, 0) };
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}
