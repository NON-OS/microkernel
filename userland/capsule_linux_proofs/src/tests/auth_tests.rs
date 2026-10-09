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

//! Package authentication, against an index and a package laid out the way
//! Alpine lays them out (see vectors/auth/make_vectors.py).

use rsa::pkcs8::DecodePublicKey;
use rsa::{Pkcs1v15Sign, RsaPublicKey};

use crate::install::auth::keys::spki;
use crate::install::auth::package::verified;
use crate::install::auth::signature::signed_index;
use crate::install::index::Index;
use crate::install::tar::entries;

pub(super) const INDEX: &[u8] = include_bytes!("../../vectors/auth/APKINDEX.tar.gz");
pub(super) const APK: &[u8] = include_bytes!("../../vectors/auth/hello.apk");
const TEST_KEY: &[u8] = include_bytes!("../../vectors/auth/test_key.spki");
const NAMED: &[u8] = b"alpine-devel@lists.alpinelinux.org-6165ee59.rsa.pub";

/// The crypto service's SHA-1 check, with the vectors' signer standing in for
/// Alpine. The key the capsule looked up must still be the one the entry names.
pub(super) fn rsa(key: &[u8], sig: &[u8], hashid: u8, digest: &[u8]) -> bool {
    assert_eq!(Some(key.to_vec()), spki(NAMED), "the capsule looked up another key");
    let Ok(test) = RsaPublicKey::from_public_key_der(TEST_KEY) else {
        return false;
    };
    hashid == 3 && test.verify(Pkcs1v15Sign::new::<sha1::Sha1>(), digest, sig).is_ok()
}

/// The checksum the signed index records for `hello`.
pub(super) fn record() -> [u8; 20] {
    let tar = signed_index(INDEX, &rsa).expect("the index must verify");
    let body = entries(&tar).into_iter().find(|e| e.name == b"APKINDEX").expect("APKINDEX");
    let index = Index::parse(&body.body);
    index.by_name("hello").and_then(|p| p.checksum).expect("a checksum for hello")
}

#[test]
fn a_signed_index_opens_and_carries_the_package_checksum() {
    let _ = record();
}

#[test]
fn a_package_matching_its_record_yields_its_files() {
    let files = verified(APK, &record()).expect("the package must verify");
    let found = entries(files.files());
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].name, b"usr/bin/hello");
}

#[test]
fn every_embedded_alpine_key_decodes_to_a_public_key() {
    for (id, len) in [("4a6a0840", 294), ("5261cecb", 294), ("6165ee59", 550)] {
        let name = format!("alpine-devel@lists.alpinelinux.org-{id}.rsa.pub");
        let der = spki(name.as_bytes()).expect("the key must decode");
        assert_eq!(der.len(), len, "{id}");
        assert!(RsaPublicKey::from_public_key_der(&der).is_ok(), "{id}");
    }
}
