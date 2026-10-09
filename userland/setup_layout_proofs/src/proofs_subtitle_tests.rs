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

//! The installer's proofs screen says what the kernel checked: a STARK proof
//! only when one ran, Merkle paths alone on a development kernel.

use crate::installer::full::proofs_subtitle::proofs_subtitle;

#[test]
fn a_development_kernel_claims_no_stark() {
    let s = proofs_subtitle(true, true, true);
    assert_eq!(s, "What this development boot checked: Merkle paths, no STARK proof.");
    assert!(!s.starts_with("Every STARK"));
}

#[test]
fn a_release_kernel_names_its_stark_proofs() {
    assert_eq!(
        proofs_subtitle(true, true, false),
        "Every STARK proof this boot checked, and what each one proved."
    );
}

#[test]
fn no_proof_and_no_report_claim_nothing() {
    assert_eq!(proofs_subtitle(true, false, false), "What this boot checked. No STARK proof is on record.");
    assert_eq!(proofs_subtitle(true, false, true), "What this boot checked. No STARK proof is on record.");
    for proof in [false, true] {
        for path in [false, true] {
            assert_eq!(
                proofs_subtitle(false, proof, path),
                "The kernel did not report what this boot checked."
            );
        }
    }
}
