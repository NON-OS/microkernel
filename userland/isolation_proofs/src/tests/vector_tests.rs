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

//! The shipped vector, vectors/honest.{proof,words}, written by
//! isolation_vector: the sample family's words, and a proof nox_verify
//! accepts under the pinned STATEMENT. The owner's attest step reads them.

use isolation_stark::Statement;
use nonos_proof_abi::isolation::STATEMENT as PIN;

use crate::sample::witness;

const PROOF: &[u8] = include_bytes!("../../vectors/honest.proof");
const WORDS: &[u8] = include_bytes!("../../vectors/honest.words");

#[test]
fn the_shipped_vector_is_the_samples_and_verifies_under_the_pin() {
    let words: Vec<u64> =
        WORDS.chunks(8).map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect();
    let st = Statement::of(&witness()).expect("the sample proves");
    assert_eq!(words, st.words());
    assert_eq!(nox_verify::verify_why(&PIN, PROOF, &st.words()), Ok(()));
    let mut other = st.words();
    other[7] ^= 1;
    assert!(nox_verify::verify(&PIN, PROOF, &other).is_err());
}
