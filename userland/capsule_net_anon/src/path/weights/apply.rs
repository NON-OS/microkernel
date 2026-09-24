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

//! Choosing which published weight applies to a relay in a position.

use super::super::relay::Flags;
use super::types::{dual, Position, Weights};

/*
 * A relay's weight depends on the position and on whether it could also have
 * served the other positions. A relay flagged both Guard and Exit is scarce,
 * so the consensus discounts it wherever it is used, and the `*d` weights are
 * the ones that do that. Getting this wrong does not break a circuit, it
 * quietly overloads the relays the network is trying to spare.
 */
/// The weight to scale a relay's bandwidth by, in units of ten thousand.
pub fn weight_for(weights: &Weights, flags: &Flags, position: Position) -> u32 {
    let both = dual(flags);
    match position {
        Position::Guard if both => weights.wgd,
        Position::Guard => weights.wgg,
        Position::Exit if both => weights.wed,
        Position::Exit => weights.wee,
        Position::Middle if both => weights.wmd,
        Position::Middle if flags.guard => weights.wmg,
        Position::Middle if flags.exit => weights.wme,
        Position::Middle => weights.wmm,
    }
}
