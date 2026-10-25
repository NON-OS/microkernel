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

//! How the enter model's wipes end as built (L78): each fate of the device
//! ends its wipe as the kernel's rules say it must, and every stepper the
//! kernel has (a release or exit, the tick, a claimant's enter) ends one in
//! some order, while no claimant ever steps one from inside Making; and both
//! answers a claimant may get before any entry of its own are reached.

use super::enter_events::{AGAIN, BY_A, BY_CLAIM, BY_TICK, BY_WAIT, CLEAN, LOST, TAKEN, UNRESET};
use super::enter_tests::built;

#[test]
fn each_fate_ends_its_wipe_as_the_rules_say() {
    for (name, _, f) in built() {
        let want = match *name {
            "clean" | "display" => CLEAN,
            "no way" | "ignores" => UNRESET,
            "late" => LOST | CLEAN,
            _ => LOST,
        };
        assert_eq!(f.events & (CLEAN | UNRESET | LOST), want, "{name}");
    }
}

#[test]
fn every_stepper_ends_a_wipe_and_none_from_inside_making() {
    let by = BY_A | BY_TICK | BY_CLAIM;
    for (name, _, f) in built() {
        assert_eq!(f.events & (by | BY_WAIT), by, "{name}");
    }
}

#[test]
fn claimants_are_told_to_wait_and_told_it_is_taken_each_held_to_i9_and_i10() {
    for (name, _, f) in built() {
        assert_eq!(f.events & (AGAIN | TAKEN), AGAIN | TAKEN, "{name}");
    }
}
