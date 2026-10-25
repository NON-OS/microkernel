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

//! Every interleaving of the actors' steps, searched depth first. A state
//! is searched once and the runs that go on from it are kept, so the runs
//! counted are every interleaving, not only the states.

use std::collections::HashMap;

use super::actors::World;
use super::check;
use super::events::{Actor, Event};
use super::gate::Gate;
use super::moves::{apply, moves};
use super::outcome::{Broken, Covered, Failure};

struct Search<'a> {
    g: &'a Gate,
    ignore: &'a [Broken],
    seen: HashMap<World, u128>,
    trace: Vec<(Actor, Event)>,
    steps: usize,
}

impl Search<'_> {
    fn keep(&self, broken: impl IntoIterator<Item = Broken>) -> Result<(), Failure> {
        match broken.into_iter().find(|b| !self.ignore.contains(b)) {
            Some(broken) => Err(Failure { broken, trace: self.trace.clone() }),
            None => Ok(()),
        }
    }

    fn visit(&mut self, w: &World) -> Result<u128, Failure> {
        if let Some(&runs) = self.seen.get(w) {
            return Ok(runs);
        }
        self.keep(check::state(self.g, w))?;
        let mut runs = 0;
        for m in moves(w) {
            let Some((next, e)) = apply(self.g, w, m) else { continue };
            self.steps += 1;
            self.trace.push((m.actor, e));
            self.keep(check::step(w, e))?;
            runs += self.visit(&next)?;
            self.trace.pop();
        }
        if runs == 0 {
            self.keep(check::end(w))?;
            runs = 1;
        }
        self.seen.insert(*w, runs);
        Ok(runs)
    }
}

/// Every interleaving from `start` under `g`, the rules in `ignore` aside.
pub fn search(g: &Gate, start: World, ignore: &[Broken]) -> Result<Covered, Failure> {
    let mut s = Search { g, ignore, seen: HashMap::new(), trace: Vec::new(), steps: 0 };
    let runs = s.visit(&start)?;
    Ok(Covered { states: s.seen.len(), steps: s.steps, runs })
}
