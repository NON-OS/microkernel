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

//! P.1: a version 2 policy decodes only with SERVE_ONE and its field
//! together, the field a port and a name of 1 to 32 bytes of [a-z0-9._-]
//! from a letter, zero padded, and nothing past it but zeros.

use super::sample::{policy, served};
use crate::{decode, PolicyError, LEN};

/// SERVE_ONE, bit 9 of the mask at byte 48; the port at 217, the name at 219.
const BIT_AT: usize = 49;
const NAME: usize = 219;

fn named(name: &[u8]) -> [u8; LEN] {
    let mut b = served().encode();
    b[NAME..NAME + 32].fill(0);
    b[NAME..NAME + name.len()].copy_from_slice(name);
    b
}

#[test]
fn the_bit_and_the_field_come_together_and_the_port_is_not_zero() {
    let mut b = policy().encode();
    b[BIT_AT] |= 2;
    assert_eq!(decode(&b), Err(PolicyError::Serve), "the bit in version 1");
    let mut b = served().encode();
    b[BIT_AT] &= !2;
    assert_eq!(decode(&b), Err(PolicyError::Serve), "the field without the bit");
    let mut b = served().encode();
    b[217..251].fill(0);
    assert_eq!(decode(&b), Err(PolicyError::Serve), "the bit without the field");
    let mut b = policy().encode();
    b[217..219].copy_from_slice(&5225u16.to_le_bytes());
    assert_eq!(decode(&b), Err(PolicyError::Reserved), "version 1 has no field");
    let mut b = served().encode();
    b[217..219].fill(0);
    assert_eq!(decode(&b), Err(PolicyError::Serve), "port 0");
}

#[test]
fn a_name_is_1_to_32_of_the_set_from_a_letter_with_no_nul_inside() {
    for good in [&b"a"[..], b"app.simplex", b"z0-9._", &[b'q'; 32]] {
        let s = decode(&named(good)).unwrap().serve_one().unwrap();
        assert_eq!((s.name().as_bytes(), usize::from(s.name_len)), (good, good.len()));
    }
    let upper = [&b"App.simplex"[..], b"app.Simplex", b"APP"];
    let other = [&b"0app"[..], b".app", b"-", b"app simplex", b"app/x", b"\xc3\xa9t\xc3\xa9"];
    for bad in upper.into_iter().chain(other).chain([&b""[..], b"app\0simplex", b"a\0\0b"]) {
        assert_eq!(decode(&named(bad)), Err(PolicyError::Serve), "{bad:?}");
    }
}

#[test]
fn a_33_byte_name_and_any_byte_past_the_pad_are_refused() {
    assert_eq!(decode(&named(&[b'q'; 33])), Err(PolicyError::Reserved));
    for at in 251..LEN {
        let mut b = served().encode();
        b[at] = b'a';
        assert_eq!(decode(&b), Err(PolicyError::Reserved), "byte {at}");
    }
}
