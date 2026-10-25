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

//! Writes one honest isolation proof of the sample family and its eight
//! public words (LE64) into the directory given: the proof the plan ships for
//! the owner's attest step beside program.bin and its MANIFEST.

use isolation_proofs::sample::{witness, ENTROPY};
use isolation_stark::prove;

fn main() {
    let dir = std::env::args().nth(1).expect("usage: isolation_vector <dir>");
    let (st, proof) = prove(&witness(), &ENTROPY).expect("an honest proof");
    let words: Vec<u8> = st.words().iter().flat_map(|w| w.to_le_bytes()).collect();
    std::fs::write(format!("{dir}/honest.proof"), &proof).expect("honest.proof");
    std::fs::write(format!("{dir}/honest.words"), words).expect("honest.words");
}
