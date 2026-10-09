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

//! The DMA posture is inside what the TPM signs.
//!
//! The quote covers the PCRs and the qualifying data, nothing else. A field of
//! the document that does not change the qualifying data could be edited after
//! the quote without breaking the signature, so each part of the posture must.

use crate::security::attest_doc::binding::{qualifying_data, DmaPosture};

const CHALLENGE: [u8; 32] = [7; 32];
const ROOT: [u8; 32] = [0xAB; 32];

fn posture() -> DmaPosture {
    DmaPosture { vendor: 1, enforcing: true, unconfined_grants: 13 }
}

#[test]
fn the_same_inputs_bind_to_the_same_value() {
    assert_eq!(
        qualifying_data(&CHALLENGE, &ROOT, &posture()),
        qualifying_data(&CHALLENGE, &ROOT, &posture())
    );
}

#[test]
fn every_part_of_the_posture_changes_what_is_signed() {
    let base = qualifying_data(&CHALLENGE, &ROOT, &posture());
    let variants = [
        DmaPosture { unconfined_grants: 0, ..posture() },
        DmaPosture { unconfined_grants: 12, ..posture() },
        DmaPosture { unconfined_grants: u32::MAX, ..posture() },
        DmaPosture { enforcing: false, ..posture() },
        DmaPosture { vendor: 0, ..posture() },
        DmaPosture { vendor: 2, ..posture() },
    ];
    for v in variants {
        assert_ne!(
            base,
            qualifying_data(&CHALLENGE, &ROOT, &v),
            "{v:?} binds like {:?}",
            posture()
        );
    }
}

#[test]
fn the_challenge_and_the_root_still_change_what_is_signed() {
    let base = qualifying_data(&CHALLENGE, &ROOT, &posture());
    assert_ne!(base, qualifying_data(&[8; 32], &ROOT, &posture()));
    assert_ne!(base, qualifying_data(&CHALLENGE, &[0xAC; 32], &posture()));
}
