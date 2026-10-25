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

//! The capsule's pinned check (pinned::forms, which pinned::held runs on a
//! proof) on each vector's pinned proof.json over the synthetic top: every
//! line passes.

use super::json::{failed, lines, parts, passed};
use super::top::SYNTHETIC;
use crate::pinned;
use crate::vectors::{self, Vector, ALL};

/// The check of `real`'s pinned proof.json, held to `pinned`'s forms.
pub(super) fn check(real: &Vector, pinned: &Vector, top: &[u8]) -> (bool, String) {
    let (bytes, publics) = parts(real);
    let json = std::str::from_utf8(real.proof_json).expect("proof.json is UTF-8");
    lines(|out| pinned::forms(out, pinned, json, &bytes, &publics, top))
}

/// `v` with its pinned forms replaced.
pub(super) fn pinned_as(v: &Vector, proof_json: Vec<u8>, format7: Vec<u8>) -> Vector {
    let (proof_json, format7) = (proof_json.leak(), format7.leak());
    Vector { proof_json, format7, ..*v }
}

#[test]
fn every_vector_is_held_over_the_synthetic_top() {
    for v in &ALL {
        assert_eq!(vectors::entropy(v).map(|e| e.len()), Some(nox_prover::ENTROPY_BYTES));
        let (ok, text) = check(v, v, SYNTHETIC);
        assert!(ok && failed(&text).is_empty(), "{text}");
        let format7 =
            format!("{}: proof-format7.bin byte for byte ({} bytes)", v.name, v.format7.len());
        assert_eq!(passed(&text).len(), 3, "{text}");
        assert!(passed(&text).contains(&format7.as_str()), "{text}");
    }
}
