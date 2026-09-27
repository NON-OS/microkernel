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

//! The RSA request the capsule sends for an OpenPGP signature, checked the
//! way the crypto service checks it, against GnuPG's own signatures.

use nonos_openpgp::{keys, verify, Material, Verifier};
use rsa::pkcs8::DecodePublicKey;
use rsa::sha2::{Sha256, Sha512};
use rsa::traits::PublicKeyParts;
use rsa::{BigUint, Pkcs1v15Sign, RsaPublicKey};

use crate::install::pacman::request::request;

const VECTORS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../openpgp/tests/vectors");

/// capsule_crypto's rsa_scheme for scheme 0, standing in for the IPC call.
struct Service;

impl Verifier for Service {
    fn rsa(&self, n: &[u8], e: &[u8], sig: &[u8], hash: u8, digest: &[u8]) -> bool {
        let Some(r) = request(n, e, sig, hash) else { return false };
        let Ok(key) = RsaPublicKey::from_public_key_der(&r.spki) else { return false };
        match r.hashid {
            0 => key.verify(Pkcs1v15Sign::new::<Sha256>(), digest, &r.sig).is_ok(),
            2 => key.verify(Pkcs1v15Sign::new::<Sha512>(), digest, &r.sig).is_ok(),
            _ => false,
        }
    }

    fn ed25519(&self, _: &[u8; 32], _: &[u8; 64], _: &[u8]) -> bool {
        false
    }
}

fn read(name: &str) -> Vec<u8> {
    std::fs::read(format!("{VECTORS}/{name}")).expect(name)
}

#[test]
fn the_spki_carries_exactly_the_key() {
    let ring = keys(&read("rsa.pub")).expect("rsa.pub");
    let Material::Rsa { n, e } = &ring[0].material else { panic!("not RSA") };
    let r = request(n, e, &[1], 8).expect("a request");
    let key = RsaPublicKey::from_public_key_der(&r.spki).expect("DER the service parses");
    assert_eq!(key.n(), &BigUint::from_bytes_be(n));
    assert_eq!(key.e(), &BigUint::from_bytes_be(e));
    assert_eq!(r.sig.len(), n.len(), "padded to the modulus width");
}

#[test]
fn gnupg_rsa_signatures_verify_through_the_request() {
    let data: Vec<u8> = (0..70000u32).map(|i| ((i * 131 + 17) % 251) as u8).collect();
    let ring = keys(&read("rsa.pub")).expect("rsa.pub");
    for sig in ["rsa-sha256.sig", "rsa-sha512.sig"] {
        assert!(verify(&Service, &ring, &read(sig), &data).is_ok(), "{sig}");
    }
    let mut changed = data.clone();
    changed[0] ^= 1;
    assert!(verify(&Service, &ring, &read("rsa-sha256.sig"), &changed).is_err());
}
