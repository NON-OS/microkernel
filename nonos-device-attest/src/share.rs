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

//! A device proof in format 7, the shared-path form L1-Shield's adapter reads
//! (A7): the proof is verified first, its query positions give the paths to
//! share, and the shared bytes are checked again before they leave.

use alloc::string::ToString;
use alloc::vec::Vec;
use stark_proofs::crypto::stark::air::{
    domain_params_blown, periodic_root, share_paths, stark_verify_ext_rounds_positions,
    stark_verify_ext_rounds_shared_why, WiredMultiGen,
};
use stark_proofs::proof_wire::{
    deserialize_rounds, deserialize_rounds_shared, serialize_rounds_shared, ParamSet,
};

use crate::circuit::build;
use crate::params::{EXTRA_BLOWUP_BITS as EXTRA, GRIND_BITS, N_QUERIES};
use crate::prove::{Error, Proof};
use crate::statement::Statement;

fn point(st: &Statement) -> Result<(WiredMultiGen, ParamSet), Error> {
    let air = build(st, None).ok_or(Error::Shape)?.wired;
    let params = ParamSet::of(&air, N_QUERIES, GRIND_BITS, EXTRA);
    Ok((air, params))
}

fn refused(why: &str) -> Error {
    Error::NotVerified(why.to_string())
}

/// The format 7 bytes of a proof of `st`, or why not: a proof that does not
/// verify is never shared.
pub fn share(st: &Statement, proof: &Proof) -> Result<Vec<u8>, Error> {
    let (air, params) = point(st)?;
    let rounds = deserialize_rounds(&proof.bytes, &params)
        .ok_or_else(|| refused("not a proof of this circuit at shape A"))?;
    let (root, log_n) = (periodic_root(&air, EXTRA), domain_params_blown(&air, EXTRA).0);
    let words = st.publics();
    let at = stark_verify_ext_rounds_positions(
        air, &rounds, N_QUERIES, GRIND_BITS, EXTRA, &root, &words,
    )
    .map_err(refused)?;
    let shared = share_paths(&rounds, &at, log_n).ok_or(Error::Prover)?;
    let bytes = serialize_rounds_shared(&rounds, &shared, &params);
    verify_shared(st, &bytes)?;
    Ok(bytes)
}

/// Check format 7 bytes against the statement alone, as the adapter will.
pub fn verify_shared(st: &Statement, bytes: &[u8]) -> Result<(), Error> {
    let (air, params) = point(st)?;
    let (skeleton, streams) = deserialize_rounds_shared(bytes, &params)
        .ok_or_else(|| refused("not a format 7 proof of this circuit at shape A"))?;
    let root = periodic_root(&air, EXTRA);
    let words = st.publics();
    stark_verify_ext_rounds_shared_why(
        air, &skeleton, &streams, N_QUERIES, GRIND_BITS, EXTRA, &root, &words,
    )
    .map_err(refused)
}
