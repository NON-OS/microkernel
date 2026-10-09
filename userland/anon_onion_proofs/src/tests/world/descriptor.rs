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


//! A v3 descriptor as the fork's hs_descriptor.c encodes one, written with
//! sha3, aes and ed25519-dalek.

use std::string::String;
use std::{format, vec};
use std::vec::Vec;

use aes::Aes256;
use base64::engine::general_purpose::{STANDARD, STANDARD_NO_PAD};
use base64::Engine;
use ctr::cipher::{KeyIvInit, StreamCipher};
use ed25519_dalek::{Signer, SigningKey};
use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::{Digest, Sha3_256, Shake256};

use super::blind::Blinded;

/// One introduction point as the service lists it.
pub struct IntroListing {
    pub specs: Vec<u8>,
    pub onion_key: [u8; 32],
    pub auth: SigningKey,
    pub enc_public: [u8; 32],
}

/// What can be made wrong in a descriptor, for the failure tests.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Faults {
    pub bad_signature: bool,
    /// The service restricts its clients: one authorized key
    /// (`client_secret`), the inner layer keyed with the cookie too.
    pub client_auth: bool,
    /// The first introduction point's auth-key certificate forged.
    pub bad_first_intro_cert: bool,
    /// The service is under load and asks for a puzzle at this effort.
    pub pow_effort: Option<u32>,
}

/// The seed a service under load publishes.
pub fn pow_seed() -> [u8; 32] {
    super::keys::bytes32("pow-seed", 0)
}

pub fn cert(kind: u8, certified: &[u8; 32], signer_public: &[u8; 32], sign: impl Fn(&[u8]) -> [u8; 64]) -> Vec<u8> {
    let mut body = vec![1u8, kind];
    body.extend_from_slice(&600_000u32.to_be_bytes());
    body.push(1);
    body.extend_from_slice(certified);
    body.push(1);
    body.extend_from_slice(&32u16.to_be_bytes());
    body.extend_from_slice(&[4, 0]);
    body.extend_from_slice(signer_public);
    let signature = sign(&body);
    body.extend_from_slice(&signature);
    body
}

fn pem(kind: &str, data: &[u8]) -> String {
    let b = STANDARD.encode(data);
    let lines: Vec<&str> = b.as_bytes().chunks(64).map(|c| core::str::from_utf8(c).unwrap()).collect();
    format!("-----BEGIN {kind}-----\n{}\n-----END {kind}-----\n", lines.join("\n"))
}

fn layer(plain: &str, secret: &[u8], subcred: &[u8; 32], revision: u64, salt: [u8; 16], constant: &[u8]) -> Vec<u8> {
    let mut data = plain.as_bytes().to_vec();
    data.resize(data.len().div_ceil(10_000) * 10_000, 0);
    let mut kdf = Shake256::default();
    for part in [secret, subcred, &revision.to_be_bytes(), &salt, constant] {
        kdf.update(part);
    }
    let mut keys = [0u8; 80];
    kdf.finalize_xof().read(&mut keys);
    let mut cipher = ctr::Ctr128BE::<Aes256>::new(keys[..32].into(), keys[32..48].into());
    cipher.apply_keystream(&mut data);
    let mut mac = Sha3_256::new();
    Digest::update(&mut mac, 32u64.to_be_bytes());
    Digest::update(&mut mac, &keys[48..]);
    Digest::update(&mut mac, 16u64.to_be_bytes());
    Digest::update(&mut mac, salt);
    Digest::update(&mut mac, &data);
    let mut out = salt.to_vec();
    out.extend_from_slice(&data);
    out.extend_from_slice(&mac.finalize());
    out
}

/// The whole descriptor text.
pub fn build(blinded: &Blinded, subcred: &[u8; 32], intro: &[IntroListing], faults: Faults) -> Vec<u8> {
    let signing = SigningKey::from_bytes(&super::keys::bytes32("desc-signing", 0));
    let signing_public = signing.verifying_key().to_bytes();
    let revision = 7u64;

    let mut inner = String::from("create2-formats 2\n");
    if let Some(effort) = faults.pow_effort {
        inner += &format!("pow-params v1 {} {effort} 2030-01-01T00:00:00\n", STANDARD.encode(pow_seed()));
    }
    for (index, point) in intro.iter().enumerate() {
        inner += &format!("introduction-point {}\n", STANDARD.encode(&point.specs));
        inner += &format!("onion-key ntor {}\n", STANDARD_NO_PAD.encode(point.onion_key));
        let auth_public = point.auth.verifying_key().to_bytes();
        inner += "auth-key\n";
        let mut auth_cert = cert(0x09, &auth_public, &signing_public, |m| signing.sign(m).to_bytes());
        if faults.bad_first_intro_cert && index == 0 {
            let last = auth_cert.len() - 1;
            auth_cert[last] ^= 1;
        }
        inner += &pem("ED25519 CERT", &auth_cert);
        inner += &format!("enc-key ntor {}\n", STANDARD_NO_PAD.encode(point.enc_public));
        inner += "enc-key-cert\n";
        let cross = super::keys::bytes32("enc-cross", 0);
        inner += &pem("ED25519 CERT", &cert(0x0B, &cross, &signing_public, |m| signing.sign(m).to_bytes()));
    }
    let cookie: [u8; 16] = super::keys::bytes32("descriptor-cookie", 0)[..16].try_into().unwrap();
    let mut inner_secret = blinded.public.to_vec();
    if faults.client_auth {
        inner_secret.extend_from_slice(&cookie);
    }
    let encrypted = layer(&inner, &inner_secret, subcred, revision, [1; 16], b"hsdir-encrypted-data");

    /* The client list: sixteen entries, one real when the service restricts
     * its clients, keyed as build_descriptor_cookie_keys keys it. */
    let (ephemeral, ephemeral_public) = super::keys::x25519("desc-auth-ephemeral", 0);
    let mut middle = format!("desc-auth-type x25519\ndesc-auth-ephemeral-key {}\n", STANDARD_NO_PAD.encode(ephemeral_public));
    for i in 0u8..16 {
        let entry = if faults.client_auth && i == 5 {
            let client_public = x25519_dalek::PublicKey::from(&x25519_dalek::StaticSecret::from(client_secret()));
            let seed = ephemeral.diffie_hellman(&client_public);
            let mut kdf = Shake256::default();
            kdf.update(subcred);
            kdf.update(seed.as_bytes());
            let mut keys = [0u8; 40];
            kdf.finalize_xof().read(&mut keys);
            let iv = [0x31u8; 16];
            let mut encrypted = cookie;
            ctr::Ctr128BE::<Aes256>::new(keys[8..40].into(), &iv.into()).apply_keystream(&mut encrypted);
            (keys[..8].to_vec(), iv, encrypted)
        } else {
            ([i; 8].to_vec(), [i; 16], [i; 16])
        };
        middle += &format!(
            "auth-client {} {} {}\n",
            STANDARD_NO_PAD.encode(&entry.0),
            STANDARD_NO_PAD.encode(entry.1),
            STANDARD_NO_PAD.encode(entry.2)
        );
    }
    middle += "encrypted\n";
    middle += &pem("MESSAGE", &encrypted);
    let superencrypted = layer(&middle, &blinded.public, subcred, revision, [2; 16], b"hsdir-superencrypted-data");

    let mut doc = String::from("hs-descriptor 3\ndescriptor-lifetime 180\ndescriptor-signing-key-cert\n");
    doc += &pem("ED25519 CERT", &cert(0x08, &signing_public, &blinded.public, |m| blinded.sign(m)));
    doc += &format!("revision-counter {revision}\nsuperencrypted\n");
    doc += &pem("MESSAGE", &superencrypted);
    let mut message = b"Tor onion service descriptor sig v3".to_vec();
    message.extend_from_slice(doc.as_bytes());
    let mut signature = signing.sign(&message).to_bytes();
    if faults.bad_signature {
        signature[0] ^= 1;
    }
    doc += &format!("signature {}\n", STANDARD_NO_PAD.encode(signature));
    doc.into_bytes()
}

/// The one client secret a restricting service lists.
pub fn client_secret() -> [u8; 32] {
    super::keys::bytes32("client-auth", 0)
}
