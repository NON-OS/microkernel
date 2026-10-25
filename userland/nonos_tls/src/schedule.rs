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

use super::traffic_keys::{wipe, Secrets, TrafficKeys};

pub(super) const EMPTY_HASH: [u8; 32] = [
    0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9, 0x24,
    0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52, 0xb8, 0x55,
];

/// The handshake traffic keys, given the hash of ClientHello..ServerHello.
/// Every secret is written where it will live; the early and derived
/// secrets on the way are wiped.
pub fn handshake_keys(shared: &[u8; 32], th: &[u8; 32], suite: u16) -> Option<TrafficKeys> {
    TrafficKeys::build(suite, |s| {
        let (zero, mut early, mut derived) = ([0u8; 32], [0u8; 32], [0u8; 32]);
        let ok = super::hkdf::extract(&zero, &zero, &mut early)
            && secret(&early, b"derived", &EMPTY_HASH, &mut derived)
            && super::hkdf::extract(&derived, shared, &mut s.handshake_secret);
        wipe(&mut early);
        wipe(&mut derived);
        ok && stage(s, th, b"c hs traffic", b"s hs traffic", suite)
    })
}

/// A stage's two traffic secrets from its base, and the keys and IVs from
/// them, each written into `s` in place.
pub(super) fn stage(s: &mut Secrets, th: &[u8; 32], c: &[u8], sv: &[u8], suite: u16) -> bool {
    secret(&s.handshake_secret, c, th, &mut s.client_secret)
        && secret(&s.handshake_secret, sv, th, &mut s.server_secret)
        && key(&s.client_secret, suite, &mut s.client_key)
        && iv(&s.client_secret, &mut s.client_iv)
        && key(&s.server_secret, suite, &mut s.server_key)
        && iv(&s.server_secret, &mut s.server_iv)
}

pub(super) fn secret(base: &[u8; 32], label: &[u8], context: &[u8; 32], out: &mut [u8; 32]) -> bool {
    super::expand_label::expand_label(base, label, context, out)
}

/* Derive the record key. AES-128-GCM uses a 16-byte key, ChaCha20-Poly1305 a
 * 32-byte key; the HKDF-Expand-Label length must match, so the shorter key is
 * derived into the low bytes and the record layer reads only what its cipher
 * needs. */
pub(super) fn key(secret: &[u8; 32], suite: u16, out: &mut [u8; 32]) -> bool {
    let len = if suite == super::constants::SUITE_AES128_GCM_SHA256 { 16 } else { 32 };
    super::expand_label::expand_label(secret, b"key", &[], &mut out[..len])
}

pub(super) fn iv(secret: &[u8; 32], out: &mut [u8; 12]) -> bool {
    super::expand_label::expand_label(secret, b"iv", &[], out)
}
