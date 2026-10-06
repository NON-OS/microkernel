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

//! Alpine's own index, as the mirror served it on 2026-09-27 (v3.20 main
//! x86_64, signed 2026-07-16 by 6165ee59), through the capsule's signature
//! check with the key the capsule pins and a real RSA verifier. A failure
//! on the device that passes here is in the crypto service, not the parse.

use rsa::pkcs8::DecodePublicKey;
use rsa::{Pkcs1v15Sign, RsaPublicKey};

use crate::install::auth::keys::spki;
use crate::install::auth::signature::signed_index;

const INDEX: &[u8] = include_bytes!("../../vectors/alpine/APKINDEX-v3.20-main-x86_64.tar.gz");

fn rsa(key: &[u8], sig: &[u8], hashid: u8, digest: &[u8]) -> bool {
    let Ok(k) = RsaPublicKey::from_public_key_der(key) else {
        return false;
    };
    match hashid {
        3 => k.verify(Pkcs1v15Sign::new::<sha1::Sha1>(), digest, sig).is_ok(),
        0 => k.verify(Pkcs1v15Sign::new::<rsa::sha2::Sha256>(), digest, sig).is_ok(),
        _ => false,
    }
}

#[test]
fn the_real_index_verifies_under_the_pinned_key() {
    assert!(spki(b"alpine-devel@lists.alpinelinux.org-6165ee59.rsa.pub").is_some());
    assert!(signed_index(INDEX, &rsa).is_some(), "the capsule's check refused a good index");
}

#[test]
fn one_flipped_byte_in_the_index_is_refused() {
    let mut bad = INDEX.to_vec();
    let at = bad.len() - 100;
    bad[at] ^= 1;
    assert!(signed_index(&bad, &rsa).is_none());
}
