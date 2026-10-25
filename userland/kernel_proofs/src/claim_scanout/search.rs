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

//! Every order of the takeover model's paths (L79), depth first, each world
//! seen once. What a step broke is recorded and gone past, so what a break
//! leads to is searched too; a world where nothing can move is an end, held
//! to the end's invariants and counted by how it ended.

use std::collections::HashSet;

use super::found::Found;
use super::knob::Knob;
use super::starts::{start, Cfg};
use super::steps::step;
use super::world::END;
use super::world_state::World;

pub(super) fn sweep(starts: &[Cfg], k: Knob) -> Found {
    let mut f = Found::default();
    starts.iter().for_each(|&c| search(start(c), k, &mut f));
    f
}

fn search(first: World, k: Knob, f: &mut Found) {
    let (mut seen, mut todo) = (HashSet::new(), vec![first]);
    while let Some(w) = todo.pop() {
        if !seen.insert(w) {
            continue;
        }
        f.broken |= w.bad;
        let before = todo.len();
        for p in 0..w.pc.len() {
            let mut n = World { bad: 0, events: 0, ..w };
            if n.pc[p] < END && step(&mut n, p, k) {
                f.events |= n.events;
                todo.push(World { events: 0, ..n });
            }
        }
        if todo.len() == before {
            f.ended(&w);
        }
    }
    f.worlds += seen.len();
}
