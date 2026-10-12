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

//! Asking for a display driver while on GOP (section 12): one name a tick,
//! 500 ms apart, the gap doubling after each round that found none, to 30 s.

use crate::display::backoff::{Backoff, FIRST_MS, LAST_MS};

#[test]
fn nothing_is_asked_before_the_first_gap() {
    let mut b = Backoff::new(1000);
    assert_eq!(b.due(1000 + FIRST_MS - 1, 4), None);
    assert_eq!(b.due(1000 + FIRST_MS, 4), Some(0));
}

#[test]
fn one_name_a_tick_in_list_order() {
    /*
     * Asking every name every tick was four IPC lookups twice a second on
     * a machine that has no display driver at all.
     */
    let mut b = Backoff::new(0);
    let mut asked = Vec::new();
    for t in (0..=2000).step_by(10) {
        if let Some(i) = b.due(t, 4) {
            asked.push(i);
        }
    }
    assert_eq!(asked, [0, 1, 2, 3]);
}

#[test]
fn the_gap_doubles_after_each_full_round_and_stops_at_thirty_seconds() {
    let mut b = Backoff::new(0);
    let mut gaps = Vec::new();
    let mut now = 0;
    for _ in 0..40 {
        now = b.due_ms;
        b.due(now, 2);
        gaps.push(b.due_ms - now);
    }
    assert_eq!(&gaps[..6], [500, 1000, 1000, 2000, 2000, 4000]);
    assert!(gaps.iter().all(|&g| g <= LAST_MS));
    assert_eq!(*gaps.last().unwrap(), LAST_MS);
    assert!(now > 0);
}

#[test]
fn an_empty_list_is_never_due() {
    let mut b = Backoff::new(0);
    assert_eq!(b.due(i64::MAX, 0), None);
}

#[test]
fn bootfb_holding_the_framebuffer_starts_the_list_again_at_once() {
    let mut b = Backoff::new(0);
    for t in [500, 1000, 3000, 5000, 9000] {
        b.due(t, 5);
    }
    b.held(9100);
    assert_eq!((b.due(9100, 5), b.gap_ms), (Some(0), FIRST_MS));
}
