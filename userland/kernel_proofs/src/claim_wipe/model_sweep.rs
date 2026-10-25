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

//! Seeds swept through the world for one reset method (or none) and one
//! knob: the invariants each seed broke, the first that broke one, and the
//! cases reached, summed for the floors.

use super::method_rule::Method;
use super::steps::run;
use super::world::World;
use super::world_cases::SEEN;
use super::world_knob::Knob;

pub(super) const METHODS: [Option<Method>; 4] =
    [Some(Method::Flr), Some(Method::AfFlr), Some(Method::PmCycle), None];

pub(super) struct Sweep {
    pub seen: [u32; SEEN],
    pub broke: u16,
    /// (seed, invariants broken, the first broken's reason).
    pub first: Option<(u64, u16, &'static str)>,
}

pub(super) fn one(seed: u64, m: Option<Method>, knob: Knob) -> World {
    let mut w = World::new(seed, m, knob);
    run(&mut w);
    w
}

/// Seeds 1 to `seeds`, stopping early once an invariant in `stop` breaks.
pub(super) fn sweep(m: Option<Method>, knob: Knob, seeds: u64, stop: u16) -> Sweep {
    let mut s = Sweep { seen: [0; SEEN], broke: 0, first: None };
    for seed in 1..=seeds {
        let w = one(seed, m, knob);
        s.seen.iter_mut().zip(w.seen).for_each(|(a, b)| *a += b);
        if w.broke != 0 && s.first.is_none() {
            s.first = Some((seed, w.broke, w.why));
        }
        s.broke |= w.broke;
        if s.broke & stop != 0 {
            break;
        }
    }
    s
}
