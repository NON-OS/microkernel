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


//! HashX and Equi-X, the proof of work an onion service can ask a client for
//! before it answers an introduction (Tor proposal 327, carried unchanged by
//! the Anyone fork in `src/ext/equix`).
//!
//! A port of the reference C, interpreter only. The capsule never maps
//! memory executable, so the reference's x86 and arm64 compilers have no
//! counterpart here; they compute the same function faster, and a service
//! cannot tell which one a client used.
//!
//! Every value that reaches these functions comes from a descriptor or a
//! cell, so none of them panics on its input: indices are bounded by the
//! types, buckets by their masks, and the solver's memory is reserved
//! fallibly.

#![no_std]

extern crate alloc;

mod blake2b;
mod execute;
mod hashx;
mod program;
mod siphash;
mod solution;
mod solver;

pub use blake2b::Blake2b;
pub use hashx::HashX;
pub use solution::{verify, verify_with, Solution, VerifyError, NUM_IDX, SOLUTION_BYTES};
pub use solver::{Solver, INDEX_SPACE, MAX_SOLS};
