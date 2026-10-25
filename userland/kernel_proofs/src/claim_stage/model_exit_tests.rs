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

use std::collections::HashSet;

use super::exit_steps::step;
use super::exit_world::{code, Entry, World};
use super::types::Stage;

const ENDS: [u8; 5] = [4, 4, 5, 2, 1];

/// Whether some order goes wrong: a newer claim's device written, frames freed
/// under Bus Master, a grant left, the old entry left for ever, or no path
/// able to move before the old claim's are done.
fn wrong(w: World, new: bool, seen: &mut HashSet<World>) -> bool {
    if w.bad {
        return true;
    }
    if !seen.insert(w) {
        return false;
    }
    let mut moved = false;
    for (p, &end) in ENDS.iter().enumerate() {
        let mut n = w;
        if n.steps[p] < end && step(&mut n, p, new) {
            moved = true;
            if wrong(n, new, seen) {
                return true;
            }
        }
    }
    let ended = (0..3).all(|p| w.steps[p] >= ENDS[p]);
    !moved && (!ended || w.grant || w.entry.is_some_and(|e| e.0 == 1))
}

/// With the sibling's release, or the exit alone with the grant path; a kill
/// in every run, parking the release only where its wait `yields`.
fn start(release: bool, yields: bool) -> World {
    let entry = Some(Entry(1, code(Stage::Ready), 0, 0));
    let steps = [0, 0, if release { 0 } else { 8 }, 0, 0];
    World { entry, grant: true, bus_on: true, bad: false, yields, steps }
}

#[test]
fn a_claim_stands_until_its_exit_its_releases_and_its_paths_are_done() {
    for release in [true, false] {
        let mut seen = HashSet::new();
        assert!(!wrong(start(release, false), true, &mut seen), "release {release}");
        assert!(seen.len() > 20, "{} states", seen.len());
    }
}

#[test]
fn the_old_exit_let_a_newer_claim_in_under_its_paths() {
    assert!(wrong(start(true, false), false, &mut HashSet::new()));
    assert!(wrong(start(false, false), false, &mut HashSet::new()), "the exit alone");
}

#[test]
fn a_release_killed_in_a_wait_that_yields_kept_the_entry_for_ever() {
    assert!(wrong(start(true, true), true, &mut HashSet::new()), "L75's wait");
}
