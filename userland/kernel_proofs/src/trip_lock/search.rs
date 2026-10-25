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

//! Every world reachable from the start, and what was seen in them.

use std::collections::HashSet;

use super::world::{Knob, World, P};

/// (bad bits, stuck, worlds, a killed sleeper's lock taken back)
pub(super) fn search(k: Knob, up: bool) -> (u16, bool, usize, bool) {
    let mut seen = HashSet::new();
    let mut stack = vec![World::start(up)];
    let (mut bad, mut stuck, mut took) = (0u16, false, false);
    while let Some(w) = stack.pop() {
        if !seen.insert(w.clone()) {
            continue;
        }
        bad |= w.bad;
        took |= w.p[0] == P::Asleep && !w.alive[0] && w.p[2] == P::In;
        let next: Vec<(usize, World)> = (0..19)
            .filter_map(|a| {
                let mut n = w.clone();
                (n.step(a, k) && n != w).then_some((a, n))
            })
            .collect();
        /* Stuck: someone who still runs waits, and only kills or a reuse could help. */
        let progress = next.iter().any(|(a, _)| matches!(a, 0..=3 | 12 | 15..=18));
        stuck |= !progress && w.stuck();
        stack.extend(next.into_iter().map(|(_, n)| n));
    }
    (bad, stuck, seen.len(), took)
}

impl World {
    /// Someone who still runs has not finished.
    pub(super) fn stuck(&self) -> bool {
        (0..4).any(|i| self.alive[i] && !(i == 2 && self.stopped) && self.p[i] != P::Done)
    }
}
