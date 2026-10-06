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


//! The relay side of ntor (tor-spec 5.1.4, the fork's onion_ntor.c), and
//! the hop keys a relay runs, the client's keys mirrored.

use hmac::{Hmac, Mac};
use sha2::Sha256;
use x25519_dalek::{PublicKey, StaticSecret};

use crate::circuit::Hop;

const PROTOID: &[u8] = b"ntor-curve25519-sha256-1";

fn h(key: &[u8], parts: &[&[u8]]) -> [u8; 32] {
    let mut m = <Hmac<Sha256>>::new_from_slice(key).unwrap();
    for part in parts {
        m.update(part);
    }
    m.finalize().into_bytes().into()
}

fn tweak(suffix: &str) -> std::vec::Vec<u8> {
    let mut t = PROTOID.to_vec();
    t.extend_from_slice(suffix.as_bytes());
    t
}

/// Answer an onion skin (ID | B | X) as the relay holding `b`, with the
/// ephemeral `y`. Returns the 64 byte reply (Y | AUTH) and the relay's hop.
pub fn answer(onionskin: &[u8], b: &StaticSecret, y: &StaticSecret) -> ([u8; 64], Hop) {
    assert_eq!(onionskin.len(), 84, "an ntor onion skin is 84 bytes");
    let id = &onionskin[..20];
    let big_b: [u8; 32] = onionskin[20..52].try_into().unwrap();
    let x: [u8; 32] = onionskin[52..84].try_into().unwrap();
    assert_eq!(big_b, PublicKey::from(b).to_bytes(), "the client named this relay's ntor key");
    let big_y = PublicKey::from(y).to_bytes();
    let xy = y.diffie_hellman(&PublicKey::from(x)).to_bytes();
    let xb = b.diffie_hellman(&PublicKey::from(x)).to_bytes();

    let mut secret = std::vec::Vec::new();
    for part in [&xy[..], &xb, id, &big_b, &x, &big_y, PROTOID] {
        secret.extend_from_slice(part);
    }
    let verify = h(&tweak(":verify"), &[&secret]);
    let auth = h(&tweak(":mac"), &[&verify, id, &big_b, &big_y, &x, PROTOID, b"Server"]);
    let mut keys = [0u8; 72];
    hkdf::Hkdf::<Sha256>::new(Some(&tweak(":key_extract")), &secret)
        .expand(&tweak(":key_expand"), &mut keys)
        .unwrap();

    let mut reply = [0u8; 64];
    reply[..32].copy_from_slice(&big_y);
    reply[32..].copy_from_slice(&auth);
    (reply, Hop::new(&mirror72(&keys)))
}

/// Df | Db | Kf | Kb with each pair swapped: what the far end runs.
fn mirror72(k: &[u8; 72]) -> [u8; 72] {
    let mut m = *k;
    m[..20].copy_from_slice(&k[20..40]);
    m[20..40].copy_from_slice(&k[..20]);
    m[40..56].copy_from_slice(&k[56..72]);
    m[56..72].copy_from_slice(&k[40..56]);
    m
}

/// The onion service's side of the virtual hop: relay_crypto_init with
/// reverse set, so the 128 key bytes with each pair swapped.
pub fn service_hop(k: &[u8; 128]) -> Hop {
    let mut m = *k;
    m[..32].copy_from_slice(&k[32..64]);
    m[32..64].copy_from_slice(&k[..32]);
    m[64..96].copy_from_slice(&k[96..128]);
    m[96..128].copy_from_slice(&k[64..96]);
    Hop::onion(&m)
}
