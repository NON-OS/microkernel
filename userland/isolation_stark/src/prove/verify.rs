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

//! Verifying a proof against the circuit rebuilt from the statement alone,
//! the engine's own check. Gates and the market check the same bytes
//! through nox_verify and the statement's program image.

use alloc::string::ToString;
use alloc::vec::Vec;

use nonos_stark::air::{periodic_root, stark_verify_ext_rounds_shared_why};
use nonos_stark::field::Fp;
use stark_proofs::proof_wire::{deserialize_rounds_shared, ParamSet};

use super::error::Error;
use crate::circuit::shape;
use crate::params::{QUERIES, STATEMENT_BLOWUP_BITS as EXTRA, STATEMENT_GRIND_BITS};
use crate::statement::Statement;

pub fn verify(st: &Statement, bytes: &[u8]) -> Result<(), Error> {
    let words: Vec<Fp> = st.words().iter().map(|&v| Fp::from_u64(v)).collect();
    let air = shape(st).ok_or(Error::Shape)?;
    let params = ParamSet::of(&air, QUERIES, STATEMENT_GRIND_BITS, EXTRA);
    let (skeleton, streams) = deserialize_rounds_shared(bytes, &params)
        .ok_or_else(|| Error::NotVerified("not a format 7 proof of this circuit".to_string()))?;
    let root = periodic_root(&air, EXTRA);
    stark_verify_ext_rounds_shared_why(
        air,
        &skeleton,
        &streams,
        QUERIES,
        STATEMENT_GRIND_BITS,
        EXTRA,
        &root,
        &words,
    )
    .map_err(|why| Error::NotVerified(why.to_string()))
}
