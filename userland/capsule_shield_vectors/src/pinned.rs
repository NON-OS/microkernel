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

//! A proof held to the pinned bytes: proof.json, its format 7 form, and that verifying.

use nox_prover::Proof;

use crate::serial::line;
use crate::vectors::Vector;

/// Whether `proof` is `v`'s pinned proof in both forms, and verifies.
pub(crate) fn held(v: &Vector, proof: &Proof, top: &[u8]) -> bool {
    let json = nox_prover::to_json(proof);
    let shared = nox_prover::to_shared(&proof.bytes, &proof.publics, top);
    let same_json = json.as_bytes() == v.proof_json;
    let same_shared = shared.as_deref().ok() == Some(v.format7);
    let verifies =
        shared.as_ref().is_ok_and(|s| nox_prover::verify_shared(s, &proof.publics, top).is_ok());
    line(
        same_json,
        &format!("{}: proof.json byte for byte ({} proof bytes)", v.name, proof.bytes.len()),
    ) & line(
        same_shared,
        &format!("{}: proof-format7.bin byte for byte ({} bytes)", v.name, v.format7.len()),
    ) & line(verifies, &format!("{}: the format 7 form verifies", v.name))
}
