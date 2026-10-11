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

//! The last of the installer's refusals the other tests left untouched: an
//! index record with no name or version, and a provided name with a kind
//! other than so:, name nothing; an OpenPGP signature under a hash the
//! crypto service lacks, or wider than its modulus, asks nothing; and a
//! key too large to encode is no key.

use crate::install::index::Index;
use crate::install::pgp::request::request;
use crate::install::pgp::spki::rsa_spki;

#[test]
fn an_index_names_only_whole_records_and_bare_or_so_names() {
    let raw = b"P:jq\nV:1.7\np:cmd:jq so:libjq.so.1 jqalias\n\nP:nameless-version\n\nV:2\n";
    let index = Index::parse(raw);
    assert!(index.by_name("jq").is_some() && index.by_name("jqalias").is_some());
    assert!(index.by_lib("libjq.so.1").is_some());
    assert!(index.by_name("cmd:jq").is_none() && index.by_name("jq ").is_none());
    assert!(index.by_name("nameless-version").is_none(), "a record with no version");
}

#[test]
fn an_openpgp_request_is_only_sha256_or_sha512_and_never_wider_than_the_key() {
    let (n, e) = (vec![0xC3; 256], vec![1, 0, 1]);
    assert_eq!(request(&n, &e, &[7; 255], 8).map(|r| (r.hashid, r.sig.len())), Some((0, 256)));
    assert_eq!(request(&n, &e, &[7; 256], 10).map(|r| r.hashid), Some(2));
    for hash in [1, 2, 3, 9, 11] {
        assert!(request(&n, &e, &[7; 256], hash).is_none(), "hash {hash}");
    }
    assert!(request(&n, &e, &[7; 257], 8).is_none(), "a signature wider than the modulus");
}

#[test]
fn a_key_too_large_to_encode_is_no_key() {
    assert!(rsa_spki(&[0xC3; 0x2000], &[1, 0, 1]).is_some());
    assert!(rsa_spki(&[0xC3; 0x2001], &[1, 0, 1]).is_none());
    assert!(rsa_spki(&[0xC3; 256], &[1; 0x101]).is_none());
}
