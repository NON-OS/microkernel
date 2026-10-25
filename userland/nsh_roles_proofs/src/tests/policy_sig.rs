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

//! A table is trusted only under the key id the host pins, signed by that
//! role authority, both halves of the signature verifying over every byte
//! before it.

use super::policy::open;
use crate::build::keys::{key, K};
use crate::build::table::TableB;
use crate::build::Spoil;
use nsh_roles::{PolicyError, Unverified};

#[test]
fn a_table_signed_by_another_authority_than_the_pinned_one_is_refused() {
    let id = key(K::AuthB).id();
    let t = TableB { signer: K::AuthB, signer_id: id, ..TableB::default() };
    assert_eq!(open(&t), Err(PolicyError::Signer));
    assert_eq!(open(&TableB { anchor: id, ..t }), Ok(()));
}

#[test]
fn a_pinned_signer_that_is_not_an_authority_is_refused() {
    let id = key(K::Stranger).id();
    let t = TableB { signer: K::Stranger, signer_id: id, anchor: id, ..TableB::default() };
    assert_eq!(open(&t), Err(PolicyError::Signer));
    let id = key(K::Appr1).id();
    let t = TableB { signer: K::Appr1, signer_id: id, anchor: id, ..TableB::default() };
    assert_eq!(open(&t), Err(PolicyError::Signer));
}

#[test]
fn a_signature_by_another_key_is_refused() {
    assert_eq!(
        open(&TableB { signer: K::AuthB, ..TableB::default() }),
        Err(PolicyError::Signature)
    );
}

#[test]
fn both_halves_of_the_signature_must_verify() {
    for spoil in [Spoil::Ed, Spoil::Ml] {
        assert_eq!(open(&TableB { spoil, ..TableB::default() }), Err(PolicyError::Signature));
    }
}

#[test]
fn the_signature_covers_every_byte_before_it() {
    let t = TableB::default();
    let good = t.bytes();
    let signed = t.signed().len();
    for at in [5usize, 9, 25, 40, signed - 100, signed - 1] {
        let mut b = good.clone();
        b[at] ^= 1;
        let r = Unverified::parse(&b).and_then(|u| u.verify(&t.anchor, 0).map(|_| ()));
        assert!(r.is_err(), "byte {at}");
    }
    let mut b = good.clone();
    b[8 + 16] ^= 1; // the serial
    let u = Unverified::parse(&b).expect("still parses");
    assert_eq!(u.verify(&t.anchor, 0), Err(PolicyError::Signature));
}
