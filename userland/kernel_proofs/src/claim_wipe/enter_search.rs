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

//! Every order of the enter model's paths (L78), each world seen once. What
//! a step broke is recorded and gone past; a world where nothing can move is
//! held to the end's invariants (enter_check).

use std::collections::{BTreeSet, HashSet};

use super::enter_fates::{starts, Fate};
use super::enter_parts::{Knob, END, IN};
use super::enter_steps::step;
use super::enter_world::World;

/// What a search found: worlds seen, worlds where nothing could move, the
/// invariants broken, the steps' events, and ends with B or A2 admitted.
#[derive(Default, Debug)]
pub(super) struct Found {
    pub worlds: usize,
    pub ends: usize,
    pub broken: BTreeSet<&'static str>,
    pub events: u16,
    pub b_in: usize,
    pub a2_in: usize,
}

/// Every start of `fate` with knob `k`.
pub(super) fn sweep(fate: Fate, k: Knob) -> Found {
    let mut f = Found::default();
    starts(fate).for_each(|w| search(w, k, &mut f));
    f
}

fn search(start: World, k: Knob, f: &mut Found) {
    let (mut seen, mut todo) = (HashSet::new(), vec![start]);
    while let Some(mut w) = todo.pop() {
        if !seen.insert(w) {
            continue;
        }
        // Gone past, so what a break leads to is found too.
        f.broken.extend(w.bad.take());
        let before = todo.len();
        for p in 0..w.pc.len() {
            let mut n = World { events: 0, ..w };
            if w.pc[p] < END && step(&mut n, p, k) {
                f.events |= n.events;
                todo.push(World { events: 0, ..n });
            }
        }
        if todo.len() == before {
            ended(&w, f);
        }
    }
    f.worlds += seen.len();
}

fn ended(w: &World, f: &mut Found) {
    f.broken.extend(w.end_broken());
    f.ends += 1;
    f.b_in += usize::from(w.pc[3] == IN);
    f.a2_in += usize::from(w.pc[4] == IN);
}
