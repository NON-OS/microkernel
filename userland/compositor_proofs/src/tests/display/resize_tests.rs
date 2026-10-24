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

//! Following the host (G 6): the preferred mode at start with no hold, a
//! dragged window's sizes held 250 ms, and nothing asked twice.

use crate::display::resize_rule::{Resize, HOLD_MS};

#[test]
fn at_start_the_preferred_mode_is_asked_at_once_and_once() {
    let mut r = Resize::default();
    r.start(Some((1920, 1080)));
    assert!(r.started);
    assert_eq!(r.take(5), Some((1920, 1080)), "no hold at start");
    assert_eq!(r.take(10_000), None, "asked already");
    let mut none = Resize::default();
    none.start(None);
    assert_eq!(none.take(10_000), None, "no preferred mode, nothing wanted");
}

#[test]
fn a_host_hint_already_seen_wins_over_the_preferred_mode() {
    let mut r = Resize::default();
    r.note((1280, 800), 100);
    r.start(Some((1920, 1080)));
    assert_eq!(r.take(100 + HOLD_MS), Some((1280, 800)));
}

#[test]
fn a_dragged_window_is_followed_once_it_holds_for_250_ms() {
    let mut r = Resize { started: true, ..Resize::default() };
    r.note((1000, 700), 0);
    r.note((1010, 705), 100);
    assert!(!r.due(300), "moved at 100: held 200 ms only");
    r.note((1010, 705), 300);
    assert_eq!(r.take(350), Some((1010, 705)), "the same size again keeps its start");
    r.note((0, 0), 400);
    assert_eq!(r.want, (1010, 705), "no hint changes nothing");
    assert_eq!(r.take(5000), None);
    r.note((1010, 705), 6000);
    assert_eq!(r.take(9000), None, "the size asked last is not asked again");
}
