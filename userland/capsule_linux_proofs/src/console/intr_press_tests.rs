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

//! Ctrl+C pressed again and again: SIGINT each time, a warning on the
//! second inside two seconds, SIGKILL on the third, and the count starts
//! again after a kill or once the window has passed.

use super::intr_press::{Press, Presses, WINDOW_MS};

#[test]
fn one_press_is_only_ever_sigint() {
    let mut p = Presses::new();
    assert_eq!(p.press(1_000), Press::Interrupt);
    assert_eq!(p.press(1_000 + WINDOW_MS + 1), Press::Interrupt);
    assert_eq!(p.press(1_000 + 2 * (WINDOW_MS + 1)), Press::Interrupt);
}

#[test]
fn three_inside_the_window_kill() {
    let mut p = Presses::new();
    assert_eq!(p.press(0), Press::Interrupt);
    assert_eq!(p.press(500), Press::Warn);
    assert_eq!(p.press(WINDOW_MS), Press::Kill);
}

#[test]
fn the_window_is_two_seconds_from_the_first_kept_press() {
    let mut p = Presses::new();
    p.press(0);
    p.press(1_000);
    // The first has gone out of the window: two count, so a warning.
    assert_eq!(p.press(WINDOW_MS + 1), Press::Warn);
    assert_eq!(p.press(WINDOW_MS + 2), Press::Kill);
}

#[test]
fn a_kill_starts_the_count_again() {
    let mut p = Presses::new();
    p.press(0);
    p.press(1);
    assert_eq!(p.press(2), Press::Kill);
    assert_eq!(p.press(3), Press::Interrupt);
    assert_eq!(p.press(4), Press::Warn);
    assert_eq!(p.press(5), Press::Kill);
}

#[test]
fn a_clock_that_goes_back_or_cannot_be_read_still_ends_it() {
    let mut p = Presses::new();
    p.press(5_000);
    assert_eq!(p.press(10), Press::Warn);
    assert_eq!(p.press(0), Press::Kill);
    let mut stuck = Presses::new();
    let all: Vec<Press> = (0..3).map(|_| stuck.press(0)).collect();
    assert_eq!(all, [Press::Interrupt, Press::Warn, Press::Kill]);
}
