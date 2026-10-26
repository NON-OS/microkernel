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

//! Every world the callers can reach, walked to its end. A world with a
//! bad mark ends there; a world where nobody can move while someone has not
//! finished is STUCK as well.

use std::collections::HashSet;

use super::program::H;
use super::world::{Knob, Ph, World, STUCK};

#[derive(Debug, Default)]
pub(super) struct Found {
    pub(super) bad: u8,
    pub(super) worlds: usize,
    pub(super) ends: usize,
    /// Ends where H died asleep in a send and the others went on after it.
    pub(super) swept: usize,
}

pub(super) fn search(k: Knob) -> Found {
    let (mut seen, mut todo, mut f) = (HashSet::new(), vec![World::begin()], Found::default());
    while let Some(w) = todo.pop() {
        if !seen.insert(w.clone()) {
            continue;
        }
        f.worlds += 1;
        if w.bad != 0 {
            f.bad |= w.bad;
            continue;
        }
        let next = w.next(k);
        if next.is_empty() {
            if (0..3).all(|a| w.done(a)) {
                f.ends += 1;
                f.swept += usize::from(w.ph[H] == Ph::Dead);
            } else {
                f.bad |= STUCK;
            }
        }
        todo.extend(next);
    }
    f
}
