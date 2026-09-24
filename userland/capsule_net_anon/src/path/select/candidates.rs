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

//! Gathering the relays that may fill a position, with their weights.

extern crate alloc;

use alloc::vec::Vec;

use super::super::draw::{pick, scale};
use super::super::relay::Relay;
use super::super::weights::{weight_for, Position, Weights};
use super::eligible::{eligible, excluded, Taken};

/// The relays that may fill `position`, and the weight of each.
///
pub fn candidates<'a>(
    relays: &'a [Relay],
    weights: &Weights,
    position: Position,
    taken: &[Taken],
) -> (Vec<&'a Relay>, Vec<u64>) {
    let mut chosen = Vec::new();
    let mut scaled = Vec::new();
    for relay in relays.iter() {
        if !eligible(relay, position) || excluded(relay, taken) {
            continue;
        }
        let weight = scale(relay.weight, weight_for(weights, &relay.flags, position));
        if weight == 0 {
            continue;
        }
        chosen.push(relay);
        scaled.push(weight);
    }
    (chosen, scaled)
}

/// Draw one relay for `position`. `None` when nothing is eligible.
pub fn choose<'a>(
    relays: &'a [Relay],
    weights: &Weights,
    position: Position,
    taken: &[Taken],
    roll: u64,
) -> Option<&'a Relay> {
    let (chosen, scaled) = candidates(relays, weights, position, taken);
    chosen.get(pick(&scaled, roll)?).copied()
}
