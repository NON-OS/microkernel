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

use isolation_stark::{verify, Statement};
use nonos_proof_abi::{hist_commit, syscall_root};

use super::fixture::{proven, witness};

#[test]
fn an_honest_histogram_proves_and_its_words_are_the_records() {
    let (w, (st, proof)) = (witness(), proven());
    assert_eq!(st.hc, hist_commit(&w.hist, &w.salt).unwrap());
    assert_eq!(st.a, syscall_root(&w.set).unwrap());
    assert_eq!(verify(st, proof), Ok(()));
}

#[test]
fn the_proof_is_bound_to_every_word() {
    let (st, proof) = proven();
    for k in 0..8 {
        let mut words = st.words();
        words[k] ^= 1;
        let other =
            Statement { hc: words[..4].try_into().unwrap(), a: words[4..8].try_into().unwrap() };
        assert!(verify(&other, proof).is_err(), "word {k}");
    }
}
