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

//! The isolation statement: a STARK that, under the sealed
//! histogram's commitment Hc and the declared set's root A, no system call
//! outside the set succeeded. It carries no counts.
//!
//! The histogram and its salt are the witness and never leave the prover.
//! The proof is format 7 on the pinned engine, built only from ShieldRegion
//! kinds, so it is blinded by hide_wired and passes check_fri_rank_rounds as
//! the device proof does; nox_verify checks it against the program image.

#![no_std]

extern crate alloc;

mod circuit;
mod params;
mod prove;
mod statement;

pub use circuit::{build, shape, Built};
pub use params::{LOG_TRACE, QUERIES, STATEMENT_BLOWUP_BITS, STATEMENT_GRIND_BITS, WORDS};
#[cfg(feature = "forge")]
pub use prove::forge;
pub use prove::{prove, verify, Error};
pub use statement::{Statement, Witness};
