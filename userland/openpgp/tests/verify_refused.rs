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

//! Every way of getting a signature wrong is refused, with its own reason.

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
use nonos_openpgp::{verify, Refusal};

#[test]
fn one_changed_byte_of_the_file_is_refused() {
    let mut d = data();
    d[40000] ^= 1;
    for (name, sig) in [("ed", "ed-sha512.sig"), ("rsa", "rsa-sha256.sig")] {
        let got = verify(&Host, &ring(name), &file(sig), &d).err();
        assert_eq!(got, Some(Refusal::QuickCheck), "{name}");
    }
}

/// The check bytes still match, so only the arithmetic can catch this.
#[test]
fn a_damaged_signature_value_fails_the_arithmetic() {
    for (name, sig) in [("ed", "ed-sha512.sig"), ("rsa", "rsa-sha256.sig")] {
        let mut s = file(sig);
        let last = s.len() - 1;
        s[last] ^= 0x01;
        let got = verify(&Host, &ring(name), &s, &data()).err();
        assert_eq!(got, Some(Refusal::BadSignature), "{name}");
    }
}

#[test]
fn another_keyring_does_not_vouch() {
    let got = verify(&Host, &ring("rsa"), &file("ed-sha512.sig"), &data()).err();
    assert_eq!(got, Some(Refusal::UnknownKey));
}
