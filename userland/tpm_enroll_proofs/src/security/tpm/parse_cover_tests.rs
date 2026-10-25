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

//! A quote's coverage, as bytes: one selection and its digest, and
//! nothing after it.

use crate::security::tpm::quote::covered;

#[test]
fn a_quote_over_more_than_one_bank_or_with_a_byte_after_is_refused() {
    let mut a = vec![0xFF, 0x54, 0x43, 0x47, 0x80, 0x18, 0, 0, 0, 1, 0x42];
    a.extend([0; 17 + 8]);
    a.extend([0, 0, 0, 1, 0x00, 0x0B, 3, 0x97, 0x43, 0, 0, 32]);
    a.extend([0xD1; 32]);
    let c = covered(&a).expect("one selection");
    assert_eq!((c.hash, c.select, c.digest), (0x000B, &[0x97, 0x43, 0][..], &[0xD1; 32][..]));
    let mut two = a.clone();
    two[39] = 2;
    assert!(covered(&two).is_err(), "two banks");
    assert!(covered(&[&a[..], &[0]].concat()).is_err(), "a byte after the digest");
    assert!(covered(&a[..a.len() - 1]).is_err(), "cut");
    let mut short = a[..a.len() - 34].to_vec();
    short.extend([0, 2, 0xD1, 0xD2]);
    assert!(covered(&short).is_err(), "a digest that is not SHA-256 sized");
}
