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

//! The statement's program image, written from the circuit, verifies the
//! engine's proof through nox_verify: the bytes a gate, the market and an L1
//! evaluator read.

use crate::image::{encode, record, statement};
use isolation_stark::shape;

use super::fixture::proven;

#[test]
fn nox_verify_accepts_the_proof_over_the_image() {
    let (st, proof) = proven();
    let air = shape(st).expect("the shape");
    let stmt = statement(&air, encode(&air, &record(&air)));
    assert_eq!(nox_verify::verify_why(&stmt, proof, &st.words()), Ok(()));
    let mut words = st.words();
    words[7] += 1;
    assert!(nox_verify::verify(&stmt, proof, &words).is_err(), "another A");
    let mut forged = proof.clone();
    let mid = forged.len() / 2;
    forged[mid] ^= 1;
    assert!(nox_verify::verify(&stmt, &forged, &st.words()).is_err(), "a flipped byte");
}

#[test]
fn the_image_is_one_for_every_statement() {
    let (st, _) = proven();
    let a = shape(st).expect("the shape");
    let mut other = *st;
    other.hc[2] ^= 5;
    let b = shape(&other).expect("the shape");
    assert_eq!(encode(&a, &record(&a)), encode(&b, &record(&b)), "the words ride as publics");
}
