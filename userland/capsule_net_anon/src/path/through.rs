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

//! A path through the guard the link is already open to.

extern crate alloc;

use alloc::vec::Vec;

use super::relay::Relay;
use super::select::{choose, Taken};
use super::weights::{Position, Weights};

/// A three hop path whose first hop is `guard`, with each draw's dice from
/// `roll`.
///
/// A circuit is created over a link, so its first hop can only be the relay
/// at the far end of that link. A path drawn with a guard of its own sent
/// CREATE2 carrying some other relay's identity and ntor key, and every build
/// failed. The guard is taken first, so the exit and the middle exclude it
/// and its /16 exactly as they exclude each other.
pub fn through(
    guard: &Relay,
    relays: &[Relay],
    weights: &Weights,
    mut roll: impl FnMut() -> Option<u64>,
) -> Option<Vec<Relay>> {
    let mut taken: Vec<Taken> = Vec::with_capacity(3);
    taken.push(took(guard));
    let exit = choose(relays, weights, Position::Exit, &taken, roll()?)?.clone();
    taken.push(took(&exit));
    let middle = choose(relays, weights, Position::Middle, &taken, roll()?)?.clone();
    Some(alloc::vec![guard.clone(), middle, exit])
}

pub(super) fn took(relay: &Relay) -> Taken {
    Taken { identity: relay.rsa_identity, address: relay.address }
}
