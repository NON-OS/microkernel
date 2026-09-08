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

//! Choosing one candidate out of many, without leaning on any of them.
//!
//! A route is only as unpredictable as the draw that builds it. A biased
//! draw does not fail visibly: the route still works, and the nodes at the
//! front of the list simply carry more traffic than they should, which is a
//! lean an observer can measure and the client can never see.

/// Map four bytes of the route seed onto a candidate index.
///
/// Two properties matter and neither is speed.
///
/// The draw is wide. One byte modulo the count cannot reach past index 255
/// however long the list is, and the active gateway set is already long
/// enough for that to bite.
///
/// The reduction is unbiased. Modulo skews toward the front whenever the
/// count does not divide the draw range: with a byte and 180 candidates the
/// first 76 are chosen half again as often as the rest. The multiply-shift
/// reduction spreads the whole range across the candidates evenly, leaving a
/// residual bias under `len / 2^32`, which for any real node list is below
/// one part in twenty million.
///
/// The salt is masked so every caller names four bytes that exist. A hop that
/// panics is a route that never leaves.
pub fn draw(seed: &[u8; 32], salt: u8, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    let off = (salt as usize & 7) * 4;
    let x = u32::from_le_bytes([seed[off], seed[off + 1], seed[off + 2], seed[off + 3]]);
    ((x as u64 * len as u64) >> 32) as usize
}
