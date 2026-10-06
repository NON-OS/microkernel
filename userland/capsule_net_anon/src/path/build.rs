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

//! Drawing a whole three hop path, exit first.

extern crate alloc;

use alloc::vec::Vec;
use nonos_libc::crypto_random;

use super::relay::Relay;
use super::select::{choose, Taken};
use super::through::took;
use super::weights::{Position, Weights};

/// A whole path: guard, middle, exit, in that order.
///
pub fn draw_path(relays: &[Relay], weights: &Weights) -> Option<Vec<Relay>> {
    let mut taken: Vec<Taken> = Vec::with_capacity(3);
    let exit = choose(relays, weights, Position::Exit, &taken, roll()?)?.clone();
    taken.push(took(&exit));
    let guard = choose(relays, weights, Position::Guard, &taken, roll()?)?.clone();
    taken.push(took(&guard));
    let middle = choose(relays, weights, Position::Middle, &taken, roll()?)?.clone();
    Some(alloc::vec![guard, middle, exit])
}

/// A three hop path through `guard`, the relay the link is open to.
pub fn draw_path_through(guard: &Relay, relays: &[Relay], weights: &Weights) -> Option<Vec<Relay>> {
    super::through::through(guard, relays, weights, roll)
}

/// A middle relay for a circuit whose first and last hops are already
/// fixed, apart from each of `avoid` and its /16 as any two hops are kept
/// apart. Onion circuits end at a relay the lookup chose (an HSDir, a
/// rendezvous point, an introduction point), so only the middle is drawn.
pub fn draw_middle(relays: &[Relay], weights: &Weights, avoid: &[&Relay]) -> Option<Relay> {
    let taken: Vec<Taken> = avoid.iter().map(|r| took(r)).collect();
    choose(relays, weights, Position::Middle, &taken, roll()?).cloned()
}

fn roll() -> Option<u64> {
    let mut bytes = [0u8; 8];
    if crypto_random(bytes.as_mut_ptr(), bytes.len()) != bytes.len() as i64 {
        return None;
    }
    Some(u64::from_le_bytes(bytes))
}
