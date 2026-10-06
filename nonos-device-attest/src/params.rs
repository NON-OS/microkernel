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

//! Sizes and proof parameters.
//!
//! The boot and policy trees are the enroll tool's, depth 8. Each opening walks
//! one leaf compression and then the tree, so its chain is one step longer.

use stark_proofs::crypto::stark::air::{Poseidon, RATE};
use stark_proofs::crypto::stark::field::Fp;

pub const BOOT_DEPTH: usize = 8;

/// The device registry's depth as shipped: a million devices. Every depth up to
/// `MAX_DEVICE_DEPTH` builds at `2^LOG_TRACE`; this one is the one the verifier
/// key and the Lean model are pinned to.
pub const SHIPPED_DEVICE_DEPTH: usize = 20;
pub const KERNEL_DEPTH: usize = 8;

/// Rounds per compression, as log2: 32 full rounds, the gates' Poseidon.
pub const LOG_ROUNDS: u32 = 5;

/// Base columns under independent extension coefficients, as the Shield inner.
pub const MASK_COLUMNS: usize = 2;

/*
 * The point this statement is proven at: the v2 transcript's shape A, set by
 * the STARK lane on 2026-09-30. 80.8 provable bits on the query phase, with
 * the DEEP and fold rounds above that because this trace is smaller than the
 * pool's. The old 64/1/16 point was 65.8 provable and never ships. A proof's
 * header carries these, and the verifier refuses any other point.
 */
pub const N_QUERIES: usize = 19;
pub const GRIND_BITS: u32 = 28;
pub const EXTRA_BLOWUP_BITS: u32 = 5;

/*
 * The trace length every device proof has. The rank certificate, the Lean
 * masking counts and the per-round soundness table cover a FRI that folds at
 * least three times, and the prover's certificate refuses anything less. With
 * this circuit's constraint degree that takes 2^14 rows: at 2^13 it folds
 * twice. The statement itself is under 1,500 rows; the rest is padding.
 */
pub const LOG_TRACE: u32 = 14;

/// The inactive region that brings the stack past half of `2^LOG_TRACE` for
/// every registry depth up to `MAX_DEVICE_DEPTH`.
pub const PAD_LOG: u32 = 13;

/// Independent subsets the rank certificate may draw before a proof is given
/// up and proven again under fresh entropy, as the wallet prover does.
pub const RANK_ATTEMPTS: usize = 3;

pub fn hasher() -> Poseidon {
    Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE])
}
