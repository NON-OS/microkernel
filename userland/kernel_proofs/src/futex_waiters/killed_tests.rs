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

//! Every order of a waiter killed around its queueing, a live waiter and a
//! wake (L77e): as built, a wake reaching a queued W2 never lands on a dead
//! W1, and once all is done no queue holds W1; each undone step breaks one.

use std::collections::HashSet;

use super::killed::{step, Knob, World, KEY, W1, W2};

/// (a wake spent on the dead W1 while W2 waited, W1 left queued at the end).
fn search(k: Knob) -> (bool, bool, usize) {
    let (mut seen, mut stack) = (HashSet::new(), vec![World::default()]);
    let (mut lost, mut stranded) = (false, false);
    while let Some(w) = stack.pop() {
        if !seen.insert(w.clone()) {
            continue;
        }
        let next: Vec<World> = (0..4)
            .filter_map(|p| {
                let mut n = w.clone();
                step(&mut n, p, k).then_some(n)
            })
            .collect();
        let queued = |w: &World, p: u32| w.queue.get(&KEY).is_some_and(|q| q.contains(&p));
        for n in &next {
            let woke_now = n.steps[3] == 1 && w.steps[3] == 0;
            lost |= woke_now && n.woke == Some(W1) && w.ended && queued(&w, W2);
        }
        stranded |= next.is_empty() && queued(&w, W1);
        stack.extend(next);
    }
    (lost, stranded, seen.len())
}

#[test]
fn a_killed_waiter_spends_no_wake_and_is_left_on_no_queue() {
    let (lost, stranded, worlds) = search(Knob::Built);
    assert!(!lost && !stranded && worlds > 60, "lost {lost} stranded {stranded} in {worlds}");
    assert!(search(Knob::NoForget).1, "no sweep at the kill: a sleeper left queued");
    assert!(search(Knob::WakeCountsEnded).0, "a wake spent on the dead waiter");
    assert!(search(Knob::NoCheckAfterJoin).1, "joined after the sweep: left queued");
}
