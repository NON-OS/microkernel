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
//! A whole receipt verifies, and says what it carries.
//! One receipt, one encoding: a flip of any one byte is refused, and so is
//! every cut, every extension and every other version.

use agentproof_receipt::napr::read::{parse, Fault};
use agentproof_receipt::verify;

use crate::fixture::Fixture;

#[test]
fn any_one_byte_flipped_is_refused() {
    let mut f = Fixture::new(b"allow:/tmp/out,net:10.0.2.2:80");
    let (bytes, trust) = (f.bytes(), f.trust());
    assert!(verify(&bytes, &trust).is_ok());
    for at in 0..bytes.len() {
        for bit in [0x01, 0x80] {
            let mut b = bytes.clone();
            b[at] ^= bit;
            assert!(verify(&b, &trust).is_err(), "byte {at} bit {bit:#x} still verified");
        }
    }
}

#[test]
fn a_cut_a_tail_or_another_version_is_refused() {
    let mut f = Fixture::new(b"strict");
    let bytes = f.bytes();
    for n in 0..bytes.len() {
        assert!(parse(&bytes[..n]).is_err(), "cut at {n}");
    }
    let mut long = bytes.clone();
    long.push(0);
    assert_eq!(parse(&long), Err(Fault::Trailing));
    let mut v1 = bytes.clone();
    v1[5] = 1;
    assert_eq!(parse(&v1), Err(Fault::Head));
}
