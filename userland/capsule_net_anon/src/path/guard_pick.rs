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

//! Drawing a guard this boot has not already given up on.
//!
//! A guard is replaced after it fails to answer three times running. The
//! redraw used to avoid nothing, so a heavily weighted guard that cannot be
//! reached from here came straight back, and two such guards could take
//! turns for the rest of the boot while no circuit was ever built. The
//! guards given up on are kept, and the draw avoids each of them and its
//! /16, as a path avoids two hops in one /16.
//!
//! Pure. Held in anon_ntor_proofs.

extern crate alloc;

use alloc::vec::Vec;

use super::relay::Relay;
use super::select::{choose, Taken};
use super::weights::{Position, Weights};

/// Guards given up on that a redraw still avoids. Past this the oldest is
/// forgotten, so a long boot through a bad patch cannot shut out every guard.
pub const GIVEN_UP_MAX: usize = 8;

/// Remember `relay` as a guard this boot gave up on.
pub fn give_up(given_up: &mut Vec<Taken>, relay: &Relay) {
    if given_up.len() >= GIVEN_UP_MAX {
        given_up.remove(0);
    }
    given_up.push(Taken { identity: relay.rsa_identity, address: relay.address });
}

/// Draw a guard that avoids every guard in `given_up`. When that leaves
/// none, every guard this network offers has failed from here, which says
/// more about this machine's reach than about them: the list is cleared
/// and the draw runs over them all again.
pub fn draw_guard<'a>(
    relays: &'a [Relay],
    weights: &Weights,
    given_up: &mut Vec<Taken>,
    roll: u64,
) -> Option<&'a Relay> {
    if let Some(relay) = choose(relays, weights, Position::Guard, given_up, roll) {
        return Some(relay);
    }
    if given_up.is_empty() {
        return None;
    }
    given_up.clear();
    choose(relays, weights, Position::Guard, given_up, roll)
}

/// A link to the guard that breaks sooner than this after it opened counts
/// against the guard, as a dial it refused would. Seconds, the manager's clock.
pub const LINK_YOUNG_SECONDS: u64 = 30;

/// Whether a link opened at `opened_at` and broken at `now` died young.
/// A guard that takes the connection and then drops it every time reset
/// its failures at each open and was never replaced. Only the link's own
/// break counts: a circuit that fails at a middle or exit hop drops the
/// link too, and charging the guard for that would churn guards, which
/// shows the network more of them.
pub fn died_young(opened_at: u64, now: u64) -> bool {
    now < opened_at.saturating_add(LINK_YOUNG_SECONDS)
}

