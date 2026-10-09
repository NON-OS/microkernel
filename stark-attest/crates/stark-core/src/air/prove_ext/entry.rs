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

use super::super::spec::AirExt;
use super::super::types_ext::StarkProofExt;
use super::run::prove;
use crate::field::Fp;

/// Prove that `trace` satisfies `air` at money-grade soundness. Layout and domain
/// sizing match the base prover; `grind_bits` is the FRI proof-of-work.
pub fn stark_prove_ext<A: AirExt>(
    air: &A,
    trace: &[Fp],
    n_queries: usize,
    grind_bits: u32,
) -> StarkProofExt {
    prove(air, trace, n_queries, grind_bits, 0, &[])
}

/// The same prover, with `extra_blowup_bits` of FRI low-degree headroom. Zero is
/// the minimal rate-one-half domain used everywhere in tests; a deployment vector
/// raises it so a fixed query count reaches 128-bit soundness. The verifier must be
/// given the same value, since it recomputes the domain from the AIR.
pub fn stark_prove_ext_blown<A: AirExt>(
    air: &A,
    trace: &[Fp],
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
) -> StarkProofExt {
    prove(air, trace, n_queries, grind_bits, extra_blowup_bits, &[])
}

/// The same prover bound to `context`, which is absorbed into the transcript before
/// anything else, so the proof only verifies under the same context. This is the
/// money-grade attestation prover: the extension challenges and the raised FRI rate
/// give 128-bit soundness with the proof pinned to the capsule identity.
pub fn stark_prove_ext_blown_bound<A: AirExt>(
    air: &A,
    trace: &[Fp],
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
    context: &[u8],
) -> StarkProofExt {
    prove(air, trace, n_queries, grind_bits, extra_blowup_bits, context)
}
