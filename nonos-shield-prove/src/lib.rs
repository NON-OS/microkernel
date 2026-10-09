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

//! A NOX Shield spend, proved on the machine that holds its notes.
//!
//! One call, [`prove`], takes the spend request and the seed notes as the
//! pool's tools write them, and returns the proof the production pool's
//! verifier accepts, its public limbs, and the notes it creates. It is the
//! prove path of STARKs' `nox_prover` with the host modules it leaned on
//! (files, `/dev/urandom`, process exits) taken out, so it runs without std
//! on one core inside a capsule. Nothing here touches a clock or the
//! network: the caller supplies the randomness, and every refusal is an
//! error naming what was wrong.
//!
//! The proof is verified here, and its zero-knowledge rank condition
//! certified, before it is returned, so a caller never holds bytes the
//! chain would refuse. `tests/vectors.rs` holds this crate to the four
//! pinned production vectors byte for byte.

#![no_std]

extern crate alloc;

mod api;
mod hedge;
mod json;
mod policy;
mod spend;

pub use api::{
    prove, prove_cached, share, verify, verify_shared, Error, Phase, Progress, Proof, Rank,
    ENTROPY_BYTES, PERIODIC_CUT, PERIODIC_ROOT,
};
pub use json::{pack_u256, try_address, try_unpack_digest};
pub use policy::{is_standard, unit_for};
pub use spend::Created;
pub use stark_proofs::shield::join::publics::WORDS;
