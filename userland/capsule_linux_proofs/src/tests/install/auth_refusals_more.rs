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

//! More of what package authentication must refuse: an index or package
//! of the wrong shape, and a signature entry under a key not trusted here
//! or under no signature name. And RSA256 entries are checked over
//! SHA-256, not SHA-1.

use super::auth_tests::{APK, INDEX};
use super::tar_build::file;
use crate::install::auth::digest::sha256;
use crate::install::auth::package::verified;
use crate::install::auth::signature::{signed, signed_index};
use nonos_inflate::members_within;

thread_local! {
    /// What the RSA check was last asked: the hash id and the digest.
    static ASKED: std::cell::Cell<Option<(u8, Vec<u8>)>> = const { std::cell::Cell::new(None) };
}

const TRUSTED: &str = "alpine-devel@lists.alpinelinux.org-6165ee59.rsa.pub";

#[test]
fn an_index_or_package_of_the_wrong_shape_is_refused() {
    let first = &members_within(INDEX, 1 << 24).unwrap()[0];
    let one = &INDEX[first.start..first.end];
    let three = [INDEX, one].concat();
    let yes = |_: &[u8], _: &[u8], _: u8, _: &[u8]| true;
    assert!(signed_index(one, &yes).is_none(), "a signature with no index");
    assert!(signed_index(&three, &yes).is_none(), "a third member rode along");
    let apk_first = &members_within(APK, 1 << 24).unwrap()[0];
    assert!(verified(&APK[apk_first.start..apk_first.end], &[0; 20]).is_err());
}

#[test]
fn a_signature_entry_counts_only_under_a_trusted_key_and_its_own_hash() {
    let covered = b"the index member";
    let rsa = |_: &[u8], _: &[u8], id: u8, d: &[u8]| {
        ASKED.with(|a| a.set(Some((id, d.to_vec()))));
        true
    };
    let asked = || ASKED.with(|a| a.take());
    assert!(signed(&file(&format!(".SIGN.RSA256.{TRUSTED}"), b"sig"), covered, &rsa));
    assert_eq!(asked(), Some((0, sha256(covered).to_vec())));
    for name in [".SIGN.RSA.someone-else.rsa.pub".to_string(), format!("SIGN.RSA.{TRUSTED}")] {
        assert!(!signed(&file(&name, b"sig"), covered, &rsa), "{name}");
        assert_eq!(asked(), None, "{name}: the check was asked");
    }
}
