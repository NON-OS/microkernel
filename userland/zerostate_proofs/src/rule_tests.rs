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

//! What the store write needs, and how keeping moves within a boot: off at
//! the start, on once granted, and never off again.

use crate::keep_rule::{after, may_write};
use crate::sweep::Rng;

#[test]
fn the_store_is_shut_until_keeping_is_on() {
    assert!(!may_write(false));
    assert!(may_write(true));
}

#[test]
fn keeping_turns_on_only_when_granted() {
    assert!(!after(false, false));
    assert!(after(false, true));
}

#[test]
fn once_on_no_ask_turns_it_off() {
    assert!(after(true, false));
    assert!(after(true, true));
}

/* A boot is a run of asks from the start, off: the store opens at the first
 * granted ask and stays open, and is shut before it. */
#[test]
fn random_runs_of_asks_open_the_store_at_the_first_grant_and_keep_it_open() {
    let mut r = Rng::new(0x5eed_0005);
    for _ in 0..20_000 {
        let mut armed = false;
        let mut seen_grant = false;
        for _ in 0..r.below(40) {
            let granted = r.below(5) == 0;
            seen_grant |= granted;
            armed = after(armed, granted);
            assert_eq!(may_write(armed), seen_grant);
        }
    }
}
