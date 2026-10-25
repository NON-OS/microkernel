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

//! The rule's table, and every order of the holder, the contender and the
//! kill (L77e): as built, never two inside and never a contender left
//! waiting for good; each weakened rule breaks one of the two.

use std::collections::HashSet;

use super::transport_rule::takes_over;
use super::world::{Knob, World};

/// (some order goes bad, some order ends with C never inside, worlds seen).
fn search(k: Knob) -> (bool, bool, usize) {
    let (mut seen, mut stack, mut bad, mut stuck) =
        (HashSet::new(), vec![World::start()], false, false);
    while let Some(w) = stack.pop() {
        if !seen.insert(w) {
            continue;
        }
        bad |= w.bad;
        let next: Vec<World> = (0..3)
            .filter_map(|p| {
                let mut n = w;
                n.step(p, k).then_some(n)
            })
            .collect();
        stuck |= next.is_empty() && w.c != 2;
        stack.extend(next);
    }
    (bad, stuck, seen.len())
}

#[test]
fn only_a_holder_that_never_runs_again_is_taken_over() {
    for pid in [0, 1, 7] {
        for (ended, on_cpu) in [(false, false), (false, true), (true, false), (true, true)] {
            assert_eq!(takes_over(pid, ended, on_cpu), pid != 0 && ended && !on_cpu);
        }
    }
}

#[test]
fn a_killed_holder_strands_no_one_and_shares_with_no_one() {
    for k in [Knob::Built, Knob::NoDeadReturn] {
        let (bad, stuck, worlds) = search(k);
        assert!(!bad && !stuck && worlds > 40, "{k:?}: bad {bad} stuck {stuck} in {worlds}");
    }
    assert!(search(Knob::NoTakeover).1, "no takeover: a contender waits for good");
    assert!(search(Knob::DeadOnly).0, "taken over from a dead holder still on its CPU");
    assert!(search(Knob::OffCpuOnly).0, "taken over from a live sleeping holder");
}
