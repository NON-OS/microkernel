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
//! The starved rule's parts: the run states that live, a row born after the
//! owner fell silent read as another process, and the cap.

use crate::watch_starved::{lives, OWNER_HUNG_MS};

#[test]
fn ready_running_and_asleep_are_the_states_that_live() {
    // 0 new, 1 ready, 2 running, 3 sleeping, 4 stopped, 5 zombie, 6 ended.
    // Asleep counts: a compositor blocked on a starved peer sleeps (smp1).
    let live: Vec<u8> = (0..=255).filter(|&s| lives(s, 60_000, 3_000)).collect();
    assert_eq!(live, [1, 2, 3]);
    assert!(!lives(1, 3_000, 3_000), "born as the owner fell silent: not it");
    assert!(lives(3, 3_001, 3_000));
}

#[test]
fn the_cap_outlasts_the_longest_starved_turn_seen() {
    // One processor, 25 capsules starting: the compositor's slowest turn was
    // 3.5 s (lead, 26 October). Ten seconds holds that with room.
    assert_eq!(OWNER_HUNG_MS, 10_000);
}
