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

//! The family's isolation proof, made once per test run by isolation_stark
//! from the sealed histogram, its salt and the policy's set.

use std::sync::OnceLock;

use isolation_stark::{prove, Witness};

use super::run::Family;

pub fn isolation_proof(f: &Family) -> &'static [u8] {
    static ONE: OnceLock<([u64; 8], Vec<u8>)> = OnceLock::new();
    let (words, proof) = ONE.get_or_init(|| {
        let set = f.policy[152..217].try_into().expect("the set");
        let w = Witness { hist: f.hist, salt: f.salt, set };
        let (st, proof) = prove(&w, &[0x15; 64]).expect("an isolation proof");
        (st.words(), proof)
    });
    assert_eq!(*words, f.publics, "one family per test run");
    proof
}
