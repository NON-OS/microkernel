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

//! The loop both record sweeps share: honest plans and one of each lie
//! in turn, each decoded and held to what the plan says it must be.

use super::lie::{tell, ALL};
use super::plan::{plan, Plan};
use super::rng::Rng;
use super::widths::Widths;

/// How many honest plans were taken and how many of each lie told.
pub struct Tally {
    pub taken: usize,
    pub told: [usize; 8],
}

/// Run `rounds` plans through `decode`, which writes a plan as bytes and
/// answers what the crate read back from them.
pub fn run<F>(seed: u64, w: &Widths, rounds: usize, mut decode: F) -> Tally
where
    F: FnMut(&Plan, &mut Rng) -> Option<Plan>,
{
    let mut r = Rng::new(seed);
    let mut n = Tally { taken: 0, told: [0; 8] };
    for round in 0..rounds {
        let mut p = plan(&mut r, w);
        let Some(l) = ALL.get(round % 9).copied() else {
            assert_eq!(decode(&p, &mut r), Some(p), "an honest plan");
            n.taken += 1;
            continue;
        };
        if tell(&mut p, l, w) {
            assert!(!p.scans(), "{l:?} left {p:?} sound");
            assert_eq!(decode(&p, &mut r), None, "{l:?} {p:?}");
            n.told[l as usize] += 1;
        }
    }
    n
}
