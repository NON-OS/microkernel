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


//! The onion service: its keys, its descriptor, and how it answers an
//! INTRODUCE2 with a RENDEZVOUS1 (hs_cell.c and hs_ntor.c, service side).

use std::string::String;
use std::vec::Vec;

use aes::Aes256;
use ctr::cipher::{KeyIvInit, StreamCipher};
use ed25519_dalek::SigningKey;
use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::{Digest, Sha3_256, Shake256};
use x25519_dalek::{PublicKey, StaticSecret};

use super::blind::{blind, identity, Blinded};
use super::descriptor::{build, Faults, IntroListing};
use super::keys;

const PROTOID: &[u8] = b"tor-hs-ntor-curve25519-sha3-256-1";

fn t(suffix: &str) -> Vec<u8> {
    let mut v = PROTOID.to_vec();
    v.extend_from_slice(suffix.as_bytes());
    v
}

fn mac(key: &[u8], parts: &[&[u8]]) -> [u8; 32] {
    let mut h = Sha3_256::new();
    Digest::update(&mut h, (key.len() as u64).to_be_bytes());
    Digest::update(&mut h, key);
    for part in parts {
        Digest::update(&mut h, part);
    }
    h.finalize().into()
}

/// One introduction point's service keys.
pub struct IntroKeys {
    pub auth: SigningKey,
    pub enc: StaticSecret,
    pub enc_public: [u8; 32],
}

/// What a decrypted INTRODUCE2 asks for.
pub struct Request {
    pub client: [u8; 32],
    pub cookie: [u8; 20],
    pub rp_onion_key: [u8; 32],
    pub rp_specs: Vec<u8>,
    /// The proof-of-work extension, if the client sent one.
    pub pow: Option<PowField>,
}

/// trn_cell_extension_pow, as the service reads it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PowField {
    pub nonce: [u8; 16],
    pub effort: u32,
    pub seed_head: [u8; 4],
    pub solution: [u8; 16],
}

pub struct Service {
    pub seed: [u8; 32],
    pub identity: [u8; 32],
    pub blinded: Blinded,
    pub subcredential: [u8; 32],
    pub intro: Vec<IntroKeys>,
}

impl Service {
    pub fn new(period: u64, points: usize) -> Self {
        let seed = keys::bytes32("service-identity", 0);
        let identity = identity(&seed);
        let blinded = blind(&seed, period, 1440);
        let mut credential = Sha3_256::new();
        Digest::update(&mut credential, b"credential");
        Digest::update(&mut credential, identity);
        let mut sub = Sha3_256::new();
        Digest::update(&mut sub, b"subcredential");
        Digest::update(&mut sub, credential.finalize());
        Digest::update(&mut sub, blinded.public);
        let intro = (0..points as u32)
            .map(|i| {
                let (enc, enc_public) = keys::x25519("service-intro-enc", i);
                IntroKeys { auth: SigningKey::from_bytes(&keys::bytes32("service-intro-auth", i)), enc, enc_public }
            })
            .collect();
        Self { seed, identity, blinded, subcredential: sub.finalize().into(), intro }
    }

    /// The address, base32 by hand, with the fork's checksum prefix.
    pub fn address(&self) -> String {
        let mut c = Sha3_256::new();
        Digest::update(&mut c, b".anyone checksum");
        Digest::update(&mut c, self.identity);
        Digest::update(&mut c, [3u8]);
        let digest = c.finalize();
        let mut raw = self.identity.to_vec();
        raw.extend_from_slice(&digest[..2]);
        raw.push(3);
        let alphabet = b"abcdefghijklmnopqrstuvwxyz234567";
        let mut out = String::new();
        let (mut acc, mut bits) = (0u64, 0u32);
        for byte in raw {
            acc = (acc << 8) | byte as u64;
            bits += 8;
            while bits >= 5 {
                bits -= 5;
                out.push(alphabet[((acc >> bits) & 31) as usize] as char);
            }
        }
        out + ".anyone"
    }

    /// The descriptor, listing intro point `i` at the relay whose link
    /// specifiers and ntor key are `points[i]`.
    pub fn descriptor(&self, points: &[(Vec<u8>, [u8; 32])], faults: Faults) -> Vec<u8> {
        let listings: Vec<IntroListing> = points
            .iter()
            .zip(self.intro.iter())
            .map(|((specs, onion_key), keys)| IntroListing {
                specs: specs.clone(),
                onion_key: *onion_key,
                auth: keys.auth.clone(),
                enc_public: keys.enc_public,
            })
            .collect();
        build(&self.blinded, &self.subcredential, &listings, faults)
    }

    /// Decrypt and check an INTRODUCE1 body arriving at intro point `i`.
    pub fn introduce(&self, i: usize, body: &[u8]) -> Result<Request, &'static str> {
        let keys = &self.intro[i];
        let auth_public = keys.auth.verifying_key().to_bytes();
        if body.len() < 56 + 32 + 32 || body[..20] != [0u8; 20] || body[20] != 2 || body[21..23] != [0, 32] {
            return Err("INTRODUCE1 header");
        }
        if body[23..55] != auth_public || body[55] != 0 {
            return Err("INTRODUCE1 auth key or extensions");
        }
        let client: [u8; 32] = body[56..88].try_into().unwrap();
        let ciphertext = &body[88..body.len() - 32];
        let dh = keys.enc.diffie_hellman(&PublicKey::from(client)).to_bytes();
        let mut kdf = Shake256::default();
        for part in [&dh[..], &auth_public, &client, &keys.enc_public, PROTOID, &t(":hs_key_extract"), &t(":hs_key_expand"), &self.subcredential] {
            kdf.update(part);
        }
        let mut k = [0u8; 64];
        kdf.finalize_xof().read(&mut k);
        if mac(&k[32..], &[&body[..body.len() - 32]]) != body[body.len() - 32..] {
            return Err("INTRODUCE1 MAC");
        }
        let mut plain = ciphertext.to_vec();
        ctr::Ctr128BE::<Aes256>::new(k[..32].into(), &[0u8; 16].into()).apply_keystream(&mut plain);
        /* The extensions, by trn_extension: a count, then type, length and
         * body for each. Only the PoW one (type 2) is read. */
        let mut pow = None;
        let mut o = 21;
        for _ in 0..plain[20] {
            let (kind, len) = (plain[o], plain[o + 1] as usize);
            let data = plain.get(o + 2..o + 2 + len).ok_or("INTRODUCE1 extension length")?;
            if kind == 2 {
                if len != 41 || data[0] != 1 {
                    return Err("INTRODUCE1 PoW extension");
                }
                pow = Some(PowField {
                    nonce: data[1..17].try_into().unwrap(),
                    effort: u32::from_be_bytes(data[17..21].try_into().unwrap()),
                    seed_head: data[21..25].try_into().unwrap(),
                    solution: data[25..41].try_into().unwrap(),
                });
            }
            o += 2 + len;
        }
        if plain[o] != 1 || plain[o + 1..o + 3] != [0, 32] {
            return Err("INTRODUCE1 plaintext");
        }
        let specs_at = o + 35;
        let n = plain[specs_at] as usize;
        let mut at = specs_at + 1;
        for _ in 0..n {
            at += 2 + plain[at + 1] as usize;
        }
        if plain[at..].iter().any(|b| *b != 0) {
            return Err("INTRODUCE1 padding is not zero");
        }
        Ok(Request {
            client,
            cookie: plain[..20].try_into().unwrap(),
            rp_onion_key: plain[o + 3..o + 35].try_into().unwrap(),
            rp_specs: plain[specs_at..at].to_vec(),
            pow,
        })
    }

    /// RENDEZVOUS1's handshake for `request` through intro point `i`, with
    /// the service's ephemeral `y`, and the 128 hop key bytes. `tamper`
    /// spoils the MAC.
    pub fn rendezvous(&self, i: usize, request: &Request, y: &StaticSecret, tamper: bool) -> ([u8; 64], [u8; 128]) {
        let keys = &self.intro[i];
        let auth = keys.auth.verifying_key().to_bytes();
        let big_y = PublicKey::from(y).to_bytes();
        let x = PublicKey::from(request.client);
        let xy = y.diffie_hellman(&x).to_bytes();
        let xb = keys.enc.diffie_hellman(&x).to_bytes();
        let mut secret = Vec::new();
        for part in [&xy[..], &xb, &auth, &keys.enc_public, &request.client, &big_y, PROTOID] {
            secret.extend_from_slice(part);
        }
        let seed = mac(&secret, &[&t(":hs_key_extract")]);
        let verify = mac(&secret, &[&t(":hs_verify")]);
        let mut auth_mac = mac(&[&verify[..], &auth, &keys.enc_public, &big_y, &request.client, PROTOID, b"Server"].concat(), &[&t(":hs_mac")]);
        if tamper {
            auth_mac[0] ^= 1;
        }
        let mut kdf = Shake256::default();
        kdf.update(&seed);
        kdf.update(&t(":hs_key_expand"));
        let mut hop = [0u8; 128];
        kdf.finalize_xof().read(&mut hop);
        let mut handshake = [0u8; 64];
        handshake[..32].copy_from_slice(&big_y);
        handshake[32..].copy_from_slice(&auth_mac);
        (handshake, hop)
    }
}
