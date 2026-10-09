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

use nonos_attest_path::{parse_v4, verify, Kind};
use nox_verify::statements::ATTEST;

use super::witness::words;
use crate::context::DEPTH;

/// A v4 trailer checked as a gate checks it: a strict parse for this kind, the
/// path folded to `root`, then the proof over words computed from `ctx`.
pub fn check_v4(root: &[u8; 32], kind: Kind, ctx: &[u8], t: &[u8]) -> Result<(), String> {
    // A development enrollment's trailer is the path alone (policy::paths_only).
    if crate::policy::paths_only() && t.starts_with(&nonos_attest_path::MAGIC) {
        return if verify(root, DEPTH, kind, ctx, t) {
            Ok(())
        } else {
            Err("the path does not fold to the root".into())
        };
    }
    let v = parse_v4(t, kind, ATTEST.max_proof_bytes).ok_or("not a v4 trailer of this kind")?;
    if !verify(root, DEPTH, kind, ctx, v.path) {
        return Err("the path does not fold to the root".into());
    }
    let root_words = words(root).ok_or("root is not four canonical words")?;
    let publics = nox_verify::attest::words(root_words, ctx, kind as u64)
        .ok_or("no public words for this slot")?;
    nox_verify::verify(&ATTEST, v.proof, &publics)
        .map_err(|r| format!("the STARK was refused: {}", r.reason()))
}
