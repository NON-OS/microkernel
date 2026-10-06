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

//! What package authentication must refuse.

use super::auth_tests::{record, rsa, APK, INDEX};
use crate::install::auth::checksum;
use crate::install::auth::package::verified;
use crate::install::auth::signature::signed_index;

const UNTRUSTED: &[u8] = include_bytes!("../../vectors/auth/untrusted.tar.gz");
const SWAPPED: &[u8] = include_bytes!("../../vectors/auth/swapped.apk");

fn flipped(bytes: &[u8], at: usize) -> Vec<u8> {
    let mut out = bytes.to_vec();
    out[at] ^= 1;
    out
}

#[test]
fn a_changed_or_extended_index_is_refused() {
    for at in [20, INDEX.len() / 2, INDEX.len() - 12] {
        assert!(signed_index(&flipped(INDEX, at), &rsa).is_none(), "byte {at}");
    }
    let mut longer = INDEX.to_vec();
    longer.push(0);
    assert!(signed_index(&longer, &rsa).is_none(), "a trailing byte rode along");
}

#[test]
fn an_index_signed_under_a_key_not_trusted_here_is_refused() {
    assert!(signed_index(UNTRUSTED, &|_: &[u8], _: &[u8], _: u8, _: &[u8]| true).is_none());
}

#[test]
fn a_package_is_refused_on_any_mismatch() {
    let sum = record();
    assert!(verified(APK, &flipped(&sum, 0).try_into().unwrap()).is_none());
    assert!(verified(&flipped(APK, APK.len() - 12), &sum).is_none());
    // Honest control, other data: only the datahash can catch this one.
    assert!(verified(SWAPPED, &sum).is_none());
}

#[test]
fn a_checksum_is_q1_and_twenty_bytes_of_base64() {
    assert!(checksum("Q1VBuPqTmRFkXS59UyXcV3OwNgKi4=").is_some());
    assert!(checksum("VBuPqTmRFkXS59UyXcV3OwNgKi4=").is_none());
    assert!(checksum("Q1VBuPqTmRFkXS59UyXcV3OwNg==").is_none());
    assert!(checksum("Q1VBuP*TmRFkXS59UyXcV3OwNgKi4=").is_none());
}
