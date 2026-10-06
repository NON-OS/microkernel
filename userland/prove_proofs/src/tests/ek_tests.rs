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

//! The device is found by its TPM's endorsement key, P-256 or RSA 2048, in
//! the order given; a key not enrolled, none at all, or a malformed answer is
//! refused.

use crate::assemble::error::Refusal;
use crate::fixture::{answer, device};

#[test]
fn the_device_is_found_under_either_endorsement_key() {
    let mut d = device();
    assert!(d.assemble().is_ok(), "P-256 not enrolled, RSA enrolled");
    d.eks.swap(0, 1);
    assert!(d.assemble().is_ok(), "RSA first");
    d.eks.truncate(1);
    assert!(d.assemble().is_ok(), "RSA alone");
}

#[test]
fn an_endorsement_key_not_enrolled_is_refused() {
    let mut d = device();
    d.eks = vec![answer(0xEC)];
    assert_eq!(d.assemble().err(), Some(Refusal::NotEnrolled));
    d.eks = vec![answer(0xEC), answer(0x77)];
    assert_eq!(d.assemble().err(), Some(Refusal::NotEnrolled));
    d.eks = Vec::new();
    assert_eq!(d.assemble().err(), Some(Refusal::NoEk));
}

/// Cut, or with a size field that does not cover the area exactly, an
/// answer is refused even ahead of one that would be found.
#[test]
fn a_malformed_endorsement_key_answer_is_refused() {
    let good = device().eks[1].clone();
    for bad in
        [good[..33].to_vec(), good[..good.len() - 1].to_vec(), [good.clone(), vec![0]].concat()]
    {
        let mut d = device();
        d.eks.insert(0, bad);
        assert_eq!(d.assemble().err(), Some(Refusal::EkAnswer));
    }
}
