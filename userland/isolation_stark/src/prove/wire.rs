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

//! A proof's rounds to format 7: the verifier's query positions, the shared
//! Merkle paths, the bytes. They leave only after they verify.

use alloc::string::ToString;
use alloc::vec::Vec;

use nonos_stark::air::{
    domain_params_blown, periodic_root, share_paths, stark_verify_ext_rounds_positions,
    StarkProofExtRounds,
};
use nonos_stark::field::Fp;
use stark_proofs::proof_wire::{serialize_rounds_shared, ParamSet};

use super::error::Error;
use super::rank::certify;
use super::verify::verify;
use crate::circuit::shape;
use crate::params::{QUERIES, STATEMENT_BLOWUP_BITS as EXTRA, STATEMENT_GRIND_BITS};
use crate::statement::Statement;

pub(super) fn wire(
    st: &Statement,
    rounds: &StarkProofExtRounds,
    params: &ParamSet,
    words: &[Fp],
) -> Result<Vec<u8>, Error> {
    let air = shape(st).ok_or(Error::Shape)?;
    let root = periodic_root(&air, EXTRA);
    let log_n = domain_params_blown(&air, EXTRA).0;
    let positions = stark_verify_ext_rounds_positions(
        air,
        rounds,
        QUERIES,
        STATEMENT_GRIND_BITS,
        EXTRA,
        &root,
        words,
    )
    .map_err(|why| Error::NotVerified(why.to_string()))?;
    certify(st, rounds, words)?;
    let shared = share_paths(rounds, &positions, log_n).ok_or(Error::Prover)?;
    let bytes = serialize_rounds_shared(rounds, &shared, params);
    verify(st, &bytes)?;
    Ok(bytes)
}
