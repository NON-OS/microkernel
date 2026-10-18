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

//! The statement's fixed numbers, each in one place.

use nonos_stark::air::{Poseidon, RATE};
use nonos_stark::field::Fp;
use nonos_stark::fri_ext::ATTEST_SHAPE;

/// Poseidon at 32 rounds, the hasher the seal and every verifier use: the
/// boot attestation's LOG_ROUNDS, which the pinned engine does not export.
pub(crate) const STATEMENT_LOG_ROUNDS: u32 = 5;

/// Rate 2^-6, as every statement on the engine. Not the boot attestation's
/// EXTRA_BLOWUP_BITS: this statement sets its own.
pub const STATEMENT_BLOWUP_BITS: u32 = 5;

/// The attestation's point: 26 queries and a 28-bit grind, ATTEST_SHAPE's.
pub const QUERIES: usize = ATTEST_SHAPE.1;
pub const STATEMENT_GRIND_BITS: u32 = ATTEST_SHAPE.2;

/// Hc and A, four words each.
pub const WORDS: usize = 8;

/// Two chains, 513 gates and the publics fit 2^14 rows, as the attestation's do.
pub const LOG_TRACE: u32 = 14;

/// The two hiding columns the blinding fills.
pub(crate) const MASK_COLUMNS: usize = 2;

/// Copy constraints per product group. At the engine's default of 9 the
/// products are degree 11 and the domain 2^24; at 6 they are degree 8, two
/// more columns, and the domain 2^23 at the same rate, which is the log
/// domain L1-Shield's format-7 adapter takes.
pub(crate) const WIRE_CAP: usize = 6;

/// Fresh subsets the rank certificate may try before the prover gives up
/// and proves again, as the activity statement does.
pub(crate) const RANK_ATTEMPTS: usize = 3;

pub(crate) fn hasher() -> Poseidon {
    Poseidon::new(STATEMENT_LOG_ROUNDS, [Fp::ZERO; RATE])
}
