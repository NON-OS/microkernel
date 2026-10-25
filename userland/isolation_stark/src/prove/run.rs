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

//! Proving a statement from its witness. A proof leaves only after it
//! verifies, in format 7, the bytes the container carries.

use alloc::vec::Vec;

use nonos_stark::air::{seed_from_entropy, stark_prove_ext_rounds};
use nonos_stark::field::Fp;
use nonos_stark::fri::FRI_FOLD_LOG;
use stark_proofs::proof_wire::ParamSet;
use stark_proofs::recursion_assembly::inner::hide_wired;

use super::error::Error;
use super::wire::wire;
use crate::circuit::build;
use crate::params::{hasher, QUERIES, STATEMENT_BLOWUP_BITS as EXTRA, STATEMENT_GRIND_BITS};
use crate::statement::{Statement, Witness};

/// The statement `w` proves and its proof, blinded from `entropy`.
pub fn prove(w: &Witness, entropy: &[u8]) -> Result<(Statement, Vec<u8>), Error> {
    let st = Statement::of(w).ok_or(Error::Witness)?;
    prove_as(&st, w, entropy, |_| {}).map(|bytes| (st, bytes))
}

/// The prover under a claimed statement, with the region traces handed to
/// `tamper` before they are proven. Production claims what the witness
/// proves and tampers with nothing; a forger does both.
pub(super) fn prove_as(
    st: &Statement,
    w: &Witness,
    entropy: &[u8],
    tamper: impl FnOnce(&mut [Vec<Fp>]),
) -> Result<Vec<u8>, Error> {
    let seed = seed_from_entropy(entropy).ok_or(Error::Entropy)?;
    let words: Vec<Fp> = st.words().iter().map(|&v| Fp::from_u64(v)).collect();
    let mut b = build(st, Some(w)).ok_or(Error::Shape)?;
    tamper(&mut b.traces);
    let params = ParamSet::of(&b.wired, QUERIES, STATEMENT_GRIND_BITS, EXTRA);
    let mut witness = b.wired.trace(&b.traces);
    let radix = 1usize << FRI_FOLD_LOG;
    let blind = hide_wired(&hasher(), &b.wired, &mut witness, &seed, QUERIES, radix);
    let (rounds, _, _) = stark_prove_ext_rounds(
        b.wired,
        &mut witness,
        QUERIES,
        STATEMENT_GRIND_BITS,
        EXTRA,
        &words,
        None,
        &blind,
    )
    .ok_or(Error::Prover)?;
    drop(witness);
    wire(st, &rounds, &params, &words)
}
