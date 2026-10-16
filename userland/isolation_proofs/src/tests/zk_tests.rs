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

//! Zero knowledge, as plan/12 asks for it: the honest proof passes the
//! device proof's rank certificate, and one histogram proven under two fresh
//! 32-byte seeds opens no trace cell in common.

use std::collections::HashSet;

use isolation_stark::{
    prove, shape, QUERIES, STATEMENT_BLOWUP_BITS as EXTRA, STATEMENT_GRIND_BITS,
};
use nonos_stark::air::StarkProofExtRounds;
use nonos_stark::field::Fp;
use stark_proofs::proof_wire::{deserialize_rounds_shared, ParamSet};
use stark_proofs::zk_rank::check_fri_rank_rounds;

use super::fixture::{proven, witness};

fn rounds(proof: &[u8]) -> StarkProofExtRounds {
    let (st, _) = proven();
    let params = ParamSet::of(&shape(st).unwrap(), QUERIES, STATEMENT_GRIND_BITS, EXTRA);
    deserialize_rounds_shared(proof, &params).expect("a format 7 proof").0
}

fn opened(r: &StarkProofExtRounds) -> HashSet<u64> {
    r.pre.proof.queries.iter().flat_map(|q| q.trace.iter().map(|v| v.to_u64())).collect()
}

#[test]
fn the_honest_proof_passes_the_rank_certificate() {
    let (st, proof) = proven();
    let words: Vec<Fp> = st.words().iter().map(|&v| Fp::from_u64(v)).collect();
    let cert = check_fri_rank_rounds(shape(st).unwrap(), &rounds(proof), &words, EXTRA, 3)
        .expect("a certificate");
    assert!(cert.holds, "{cert:?}");
}

#[test]
fn two_seeds_open_no_trace_cell_in_common() {
    let (one, two) = ([0x31u8; 32], [0x32u8; 32]);
    let (st1, p1) = prove(&witness(), &one).expect("a proof under the first seed");
    let (st2, p2) = prove(&witness(), &two).expect("a proof under the second seed");
    assert_eq!(st1, st2);
    let (a, b) = (opened(&rounds(&p1)), opened(&rounds(&p2)));
    // Every query opens a row of the 35-column trace; nearly all distinct.
    assert!(a.len() > QUERIES * 35 / 2, "{} cells opened", a.len());
    let common: Vec<_> = a.intersection(&b).collect();
    assert!(common.is_empty(), "{} cells in common", common.len());
}
