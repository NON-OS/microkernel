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

//! The wallpaper service asking the compositor again: at once the first
//! time, then after a wait that doubles with each failure up to about four
//! seconds, and never given up on; a call that goes through starts the wait
//! over.

use crate::wallpaper_backoff::{Backoff, FIRST_WAIT, MAX_WAIT};

/// Turns until `b` says to try, counting the turn it does.
fn turns_to_due(b: &mut Backoff) -> u32 {
    let mut turns = 1;
    while !b.due() {
        turns += 1;
        assert!(turns <= MAX_WAIT + 1, "always due again");
    }
    turns
}

#[test]
fn the_first_try_is_at_once() {
    let mut b = Backoff::new();
    assert!(b.due());
    assert!(b.due(), "and stays due until a try fails");
}

#[test]
fn each_failure_waits_twice_as_long_up_to_the_cap_and_never_gives_up() {
    let mut b = Backoff::new();
    let mut waits = Vec::new();
    for _ in 0..12 {
        b.failed();
        waits.push(turns_to_due(&mut b) - 1);
    }
    assert_eq!(waits[..6], [FIRST_WAIT, 16, 32, 64, 128, MAX_WAIT]);
    assert!(waits[6..].iter().all(|&w| w == MAX_WAIT), "capped, still asking: {waits:?}");
}

#[test]
fn a_try_that_goes_through_starts_the_wait_over() {
    let mut b = Backoff::new();
    for _ in 0..5 {
        b.failed();
    }
    b.reset();
    assert!(b.due());
    b.failed();
    assert_eq!(turns_to_due(&mut b) - 1, FIRST_WAIT);
}
