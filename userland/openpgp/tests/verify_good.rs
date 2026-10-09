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

//! GnuPG's signatures verify, against the keys GnuPG says it made.

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
use nonos_openpgp::verify;

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02X}")).collect()
}

#[test]
fn fingerprints_match_what_gnupg_reported() {
    for line in String::from_utf8(file("fingerprints.txt")).unwrap_or_default().lines() {
        let mut words = line.split(' ');
        let name = words.next().unwrap_or_default();
        let ours: Vec<String> = ring(name).iter().map(|k| hex(&k.fingerprint)).collect();
        assert_eq!(ours, words.map(String::from).collect::<Vec<_>>(), "{name}");
    }
}

#[test]
fn every_gnupg_signature_verifies() {
    for (name, hash) in [("ed", 10), ("ed", 8), ("rsa", 8), ("rsa", 10), ("sub", 10)] {
        let sig = file(&format!("{name}-sha{}.sig", if hash == 8 { 256 } else { 512 }));
        let got = verify(&Host, &ring(name), &sig, &data());
        let ok = got.unwrap_or_else(|r| panic!("{name}/{hash}: {}", r.why()));
        assert_eq!(ok.hash, hash);
    }
}

#[test]
fn a_subkey_signature_names_the_subkey() {
    let r = ring("sub");
    let ok = verify(&Host, &r, &file("sub-sha512.sig"), &data()).map_err(|e| e.why());
    assert_eq!(ok.map(|v| v.fingerprint), Ok(r[1].fingerprint));
}
