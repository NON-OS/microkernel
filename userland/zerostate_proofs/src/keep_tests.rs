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

//! How keeping moves within a boot: off at the start, on once granted, and
//! never off again.

use crate::keep_rule::{after, may_write};
use crate::sweep::Rng;

const OLD: [u8; 8] = [9, 0, 7, 7, 0, 1, 2, 3];

#[test]
fn keeping_turns_on_once_granted_and_never_off() {
    assert!(!after(false, false));
    assert!(after(false, true));
    assert!(after(true, false));
    assert!(after(true, true));
}

/* A boot is a run of asks from the start, off: a record opens at the first
 * granted ask and stays open, and only erases pass before it. */
#[test]
fn random_runs_of_asks_open_the_store_at_the_first_grant() {
    let mut r = Rng::new(0x5eed_0005);
    for _ in 0..20_000 {
        let (mut armed, mut seen) = (false, false);
        for _ in 0..r.below(40) {
            let granted = r.below(5) == 0;
            seen |= granted;
            armed = after(armed, granted);
            assert_eq!(may_write(armed, &OLD, &[1; 8]), seen);
        }
    }
}
