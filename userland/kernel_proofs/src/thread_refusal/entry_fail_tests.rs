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

//! A failed entry leaves a kill's code standing (L77e): live_to, the one
//! store, is pinned whole, so no weaker guard lets mark_failed land over a
//! Zombie; and each entry file's failure branches all call mark_failed.

use super::entry_load_tests::body;
use super::entry_tests::SW;

// The failure branches of each entry file, in SW's order.
const FAILS: [usize; 7] = [3, 2, 2, 3, 4, 3, 4];

#[test]
fn a_failed_entry_never_stores_over_a_kill() {
    let dead = "matches!(*state, ProcessState::Zombie(_) | ProcessState::Terminated(_));";
    let live = format!("let mut state = self.state.lock(); let dead = {dead} if !dead {{");
    assert_eq!(body("fn live_to("), live + " *state = to; } !dead", "live_to weakened");
    for ((what, text, _), n) in SW.iter().zip(FAILS) {
        let failed = text.matches("pcb.mark_failed();").count();
        assert_eq!(failed, n, "{what}: a failure branch not through mark_failed");
    }
}
