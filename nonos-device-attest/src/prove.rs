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

//! Proving and verifying. A proof leaves only with its zero-knowledge
//! certificate: the masking is what hides `s`, and the certificate is what
//! shows the masking covered everything this proof reveals.

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use stark_proofs::crypto::stark::air::{
    periodic_root, seed_from_entropy, stark_prove_ext_rounds, stark_verify_ext_rounds_why,
    StarkProofExtRounds, RATE,
};
use stark_proofs::crypto::stark::field::Fp;
use stark_proofs::crypto::stark::fri::FRI_FOLD_LOG;
use stark_proofs::proof_wire::{deserialize_rounds, serialize_rounds, ParamSet};
use stark_proofs::recursion_assembly::inner::hide_wired;
use stark_proofs::zk_rank::check_fri_rank_rounds;

use crate::circuit::{build, build_with, Forge};
use crate::domain::{KIND_BOOTLOADER, KIND_KERNEL};
use crate::native::{commit, leaf, tag};
use crate::params::{hasher, EXTRA_BLOWUP_BITS, GRIND_BITS, N_QUERIES, RANK_ATTEMPTS};
use crate::statement::Statement;
use crate::witness::{Path, Witness};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The statement or witness has the wrong shape for the circuit.
    Shape,
    /// The witness does not satisfy the statement. Nothing was proven.
    Witness(&'static str),
    /// Not enough entropy for the blinding seed.
    Entropy,
    /// The prover produced nothing.
    Prover,
    /// The proof does not verify, or is not at this circuit's point.
    NotVerified(String),
    /// The proof verified but its zero-knowledge certificate was not reached.
    /// Nothing was returned: prove again with fresh entropy.
    Rank(String),
}

/// A proof as it leaves the machine: the wire bytes, header included.
pub struct Proof {
    pub bytes: Vec<u8>,
}

fn fold(start: [Fp; RATE], path: &Path) -> [Fp; RATE] {
    let h = hasher();
    path.siblings.iter().zip(&path.right).fold(start, |node, (sib, &right)| {
        if right {
            h.compress(sib, &node)
        } else {
            h.compress(&node, sib)
        }
    })
}

/// The statement, checked directly before anything is proven. A false statement
/// is refused here with a reason rather than handed to the prover.
fn check(st: &Statement, w: &Witness) -> Result<(), Error> {
    if fold(leaf(KIND_BOOTLOADER, &w.bootloader.digest), &w.bootloader.path) != st.boot_root {
        return Err(Error::Witness("the bootloader is not a slot of B"));
    }
    if fold(leaf(KIND_KERNEL, &w.kernel.digest), &w.kernel.path) != st.kernel_root {
        return Err(Error::Witness("the kernel is not a slot of P"));
    }
    if fold(commit(&w.secret), &w.device) != st.device_root {
        return Err(Error::Witness("the device is not enrolled in R"));
    }
    if tag(&w.secret, &st.scope) != st.tag {
        return Err(Error::Witness("the tag is not this device's in this scope"));
    }
    Ok(())
}

#[cfg(test)]
pub(crate) fn check_for_test(st: &Statement, w: &Witness) -> Result<(), Error> {
    check(st, w)
}

/// The parameters a proof of `st` must carry in its header.
fn params_of(st: &Statement) -> Result<ParamSet, Error> {
    let shape = build(st, None).ok_or(Error::Shape)?.wired;
    Ok(ParamSet::of(&shape, N_QUERIES, GRIND_BITS, EXTRA_BLOWUP_BITS))
}

/// Prove `st` from `w`. `entropy` comes from the refusing entropy call, fresh
/// per proof, and is used only for blinding: two proofs under one seed share a
/// blind, and their difference cancels it back to the witness. At least 48
/// bytes.
pub fn prove(st: &Statement, w: &Witness, entropy: &[u8]) -> Result<Proof, Error> {
    check(st, w)?;
    prove_forged(st, w, entropy, Forge::default())
}

#[cfg(test)]
pub(crate) fn prove_checked(st: &Statement, w: &Witness, entropy: &[u8]) -> Result<Proof, Error> {
    prove_forged(st, w, entropy, Forge::default())
}

pub(crate) fn prove_forged(
    st: &Statement,
    w: &Witness,
    entropy: &[u8],
    forge: Forge,
) -> Result<Proof, Error> {
    let seed = seed_from_entropy(entropy).ok_or(Error::Entropy)?;
    let b = build_with(st, Some(w), forge).ok_or(Error::Shape)?;
    let params = ParamSet::of(&b.wired, N_QUERIES, GRIND_BITS, EXTRA_BLOWUP_BITS);
    let mut witness = b.wired.trace(&b.traces);
    let blind =
        hide_wired(&hasher(), &b.wired, &mut witness, &seed, N_QUERIES, 1usize << FRI_FOLD_LOG);
    let publics = st.publics();
    let (rounds, _tree, _air) = stark_prove_ext_rounds(
        b.wired,
        &mut witness,
        N_QUERIES,
        GRIND_BITS,
        EXTRA_BLOWUP_BITS,
        &publics,
        None,
        &blind,
    )
    .ok_or(Error::Prover)?;
    drop(witness);
    let bytes = serialize_rounds(&rounds, &params);
    verify_rounds(st, &rounds)?;
    certify(st, &rounds)?;
    Ok(Proof { bytes })
}

/// The rank condition on the proof about to leave, over this circuit's shape.
fn certify(st: &Statement, rounds: &StarkProofExtRounds) -> Result<(), Error> {
    let shape = build(st, None).ok_or(Error::Shape)?.wired;
    let r = check_fri_rank_rounds(shape, rounds, &st.publics(), EXTRA_BLOWUP_BITS, RANK_ATTEMPTS)
        .map_err(|why| Error::Rank(why.to_string()))?;
    if !r.holds {
        return Err(Error::Rank(alloc::format!(
            "{} of {} certified after {} subsets; prove again with fresh entropy",
            r.mask_rank,
            r.bound,
            r.attempts
        )));
    }
    Ok(())
}

fn verify_rounds(st: &Statement, rounds: &StarkProofExtRounds) -> Result<(), Error> {
    let shape = build(st, None).ok_or(Error::Shape)?.wired;
    let root = periodic_root(&shape, EXTRA_BLOWUP_BITS);
    stark_verify_ext_rounds_why(
        shape,
        rounds,
        N_QUERIES,
        GRIND_BITS,
        EXTRA_BLOWUP_BITS,
        &root,
        &st.publics(),
    )
    .map_err(|why| Error::NotVerified(why.to_string()))
}

/// Verify a proof's bytes against the statement alone. The header must name
/// this circuit at shape A, so a proof at a weaker point is refused before it
/// is read. The periodic root is recomputed from the shape, never taken from
/// the prover.
pub fn verify(st: &Statement, proof: &Proof) -> Result<(), Error> {
    let params = params_of(st)?;
    let rounds = deserialize_rounds(&proof.bytes, &params)
        .ok_or_else(|| Error::NotVerified("not a proof of this circuit at shape A".to_string()))?;
    verify_rounds(st, &rounds)
}
