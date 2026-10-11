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

//! The install role's check of an escrow delivery's plaintext against its
//! pin. The reference roots are plan/22's chunk_root over the samples,
//! computed with tools/nonos_linux_pack/keccak.py.

use super::escrow_pin_tests::{bytes32, sample, word, ONE_ROOT, THREE_ROOT};
use crate::install::escrow_check::{escrow_pinned, NotPinned};
use crate::install::escrow_pin::escrow_pin;

/// The reference tree: the samples' roots, and anything else a wrong one.
fn reference(plain: &[u8]) -> [u8; 32] {
    match plain {
        p if p == sample(2500, 7, 3) => bytes32(THREE_ROOT),
        p if p == sample(1024, 13, 5) => bytes32(ONE_ROOT),
        _ => [0xEE; 32],
    }
}

#[test]
fn a_delivery_passes_only_at_its_length_and_root() {
    let (three, one) = (sample(2500, 7, 3), sample(1024, 13, 5));
    let pin3 = escrow_pin(&word(THREE_ROOT, 2500)).unwrap();
    let pin1 = escrow_pin(&word(ONE_ROOT, 1024)).unwrap();
    assert_eq!(escrow_pinned(&three, &pin3, reference), Ok(()));
    assert_eq!(escrow_pinned(&one, &pin1, reference), Ok(()));
    assert_eq!(escrow_pinned(&three[..2499], &pin3, reference), Err(NotPinned::Length));
    let mut flipped = three.clone();
    flipped[1500] ^= 1;
    assert_eq!(escrow_pinned(&flipped, &pin3, reference), Err(NotPinned::Root));
    let pin_other_len = escrow_pin(&word(THREE_ROOT, 1024)).unwrap();
    assert_eq!(escrow_pinned(&one, &pin_other_len, reference), Err(NotPinned::Root));
}
