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

//! Armored keys and signatures, as a Debian repository publishes them, read
//! and verified; a damaged checksum line is refused.

#[path = "support/bignum.rs"]
mod bignum;
#[path = "support/fixture.rs"]
mod fixture;
#[path = "support/host.rs"]
mod host;
#[path = "support/modpow.rs"]
mod modpow;

use fixture::{data, file, ring};
use host::Host;
use nonos_openpgp::{dearmor, keys, verify};

#[test]
fn an_armored_key_is_the_exported_key() {
    let ring_asc = keys(&dearmor(&file("rsa.asc")).expect("armor")).expect("keys");
    let fprs = |r: &[nonos_openpgp::Key]| r.iter().map(|k| k.fingerprint).collect::<Vec<_>>();
    assert_eq!(fprs(&ring_asc), fprs(&ring("rsa")));
}

#[test]
fn an_armored_signature_verifies() {
    let sig = dearmor(&file("rsa-sha256.asc")).expect("armor");
    assert!(verify(&Host, &ring("rsa"), &sig, &data()).is_ok());
}

#[test]
fn a_damaged_checksum_line_is_refused() {
    let text = String::from_utf8(file("rsa-sha256.asc")).expect("ascii");
    let line = text.lines().find(|l| l.starts_with('=')).expect("a checksum line");
    let flipped = format!("={}", if &line[1..2] == "A" { "B" } else { "A" }) + &line[2..];
    assert!(dearmor(text.replace(line, &flipped).as_bytes()).is_none());
}

#[test]
fn text_without_armor_is_nothing() {
    assert!(dearmor(b"just text\n").is_none());
    assert!(dearmor(b"-----BEGIN PGP SIGNATURE-----\n\nAAAA\n").is_none(), "no END line");
}
