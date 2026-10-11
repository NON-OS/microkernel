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

//! The device half of a market proof: the registry's root must be a
//! registrar root trust pins, at its depth, inside its life on the proving
//! day; the scope is the listing's market scope for that day, and the context
//! the record's own ctx. No caller argument names either.

use nonos_proof_abi::trust::DeviceRoot;
use nonos_proof_abi::{ctx, market_scope, Kind, RECORD_BYTES};
use nonos_stark::air::RATE;
use nonos_stark::field::Fp;

/// The pinned registrar a rebuilt registry's root and depth are, on `day`.
pub fn registrar(
    roots: &[DeviceRoot],
    day: u64,
    root: [Fp; RATE],
    depth: usize,
) -> Option<DeviceRoot> {
    roots
        .iter()
        .find(|r| r.good_on(day) && r.depth == depth && r.root.map(Fp::from_u64) == root)
        .copied()
}

/// A registrar's root as the statement's 32 bytes, four LE words.
pub fn root_bytes(r: &DeviceRoot) -> [u8; 32] {
    let mut b = [0u8; 32];
    for (i, w) in r.root.iter().enumerate() {
        b[8 * i..8 * i + 8].copy_from_slice(&w.to_le_bytes());
    }
    b
}

/// The scope `e` and context words the device proof binds: only the prover
/// derives them, from the listing, the day and the sealed record.
pub fn scope_and_context(
    listing_id: &[u8; 32],
    day: u64,
    raw: &[u8; RECORD_BYTES],
) -> ([Fp; 2], [Fp; RATE]) {
    let e = market_scope(listing_id, day).map(Fp::from_u64);
    (e, ctx(Kind::Isolation, raw).map(Fp::from_u64))
}
