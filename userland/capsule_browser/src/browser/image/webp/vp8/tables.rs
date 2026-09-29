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

use super::bmodes_a::BMODES_A;
use super::bmodes_b::BMODES_B;
use super::probs_a::PROBS_A;
use super::probs_b::PROBS_B;
use super::update_a::UPDATE_A;
use super::update_b::UPDATE_B;

/// Coefficient position to band (RFC 6386 13.3), with a guard entry.
pub(super) const BANDS: [usize; 17] = [0, 1, 2, 3, 6, 4, 5, 6, 6, 6, 6, 6, 6, 6, 6, 7, 0];

/// Coefficient position to its raster index in the 4x4 block.
pub(super) const ZIGZAG: [usize; 16] = [0, 1, 4, 8, 5, 2, 3, 6, 9, 12, 13, 10, 7, 11, 14, 15];

/// Extra-bit probabilities of the DCT_CAT3..DCT_CAT6 tokens.
pub(super) const CATS: [&[u8]; 4] = [
    &[173, 148, 140],
    &[176, 155, 140, 135],
    &[180, 157, 141, 134, 130],
    &[254, 254, 243, 230, 196, 177, 153, 140, 133, 130, 129],
];

/// Index of flattened token probability [type][band][context][node].
pub(super) fn prob_index(t: usize, band: usize, ctx: usize) -> usize {
    ((t * 8 + band) * 3 + ctx) * 11
}

pub(super) fn default_prob(i: usize) -> u8 {
    if i < PROBS_A.len() {
        PROBS_A[i]
    } else {
        PROBS_B[i - PROBS_A.len()]
    }
}

pub(super) fn update_prob(i: usize) -> u8 {
    if i < UPDATE_A.len() {
        UPDATE_A[i]
    } else {
        UPDATE_B[i - UPDATE_A.len()]
    }
}

/// The nine tree probabilities of a 4x4 mode given the modes above and
/// to the left (RFC 6386 12.3).
pub(super) fn bmode_probs(above: usize, left: usize) -> &'static [u8] {
    let i = (above.min(9) * 10 + left.min(9)) * 9;
    if i < BMODES_A.len() {
        &BMODES_A[i..i + 9]
    } else {
        &BMODES_B[i - BMODES_A.len()..i - BMODES_A.len() + 9]
    }
}
