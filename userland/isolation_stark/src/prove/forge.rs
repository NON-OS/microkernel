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

//! A forger's prover, for the host proofs only: any claimed statement, any
//! change to the region traces. A proof leaves the real prover only after it
//! verifies, so whatever this returns is what a forger could publish.

use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::error::Error;
use super::run::prove_as;
use crate::statement::{Statement, Witness};

/// The traces are, in order, H's chain, A's chain, the buckets and the
/// publics, each its own rows times its own width.
pub fn forge(
    st: &Statement,
    w: &Witness,
    tamper: impl FnOnce(&mut [Vec<Fp>]),
) -> Result<Vec<u8>, Error> {
    prove_as(st, w, &[0x46; 64], tamper)
}
