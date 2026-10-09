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

//! When net.core starts DHCP over: only when the bound link comes back after
//! going down, never on a driver too busy to answer.

use crate::link_watch::{Change, LinkWatch, WATCH_EVERY_MS};

#[test]
fn a_link_that_stays_up_changes_nothing() {
    let mut w = LinkWatch::new();
    for t in 0..10 {
        assert_eq!(w.observe(t * WATCH_EVERY_MS, 7, Some(true)), None);
    }
}

#[test]
fn a_drop_and_a_return_are_each_said_once() {
    let mut w = LinkWatch::new();
    assert_eq!(w.observe(0, 7, Some(true)), None);
    assert_eq!(w.observe(1_000, 7, Some(false)), Some(Change::Dropped));
    assert_eq!(w.observe(2_000, 7, Some(false)), None);
    assert_eq!(w.observe(3_000, 7, Some(true)), Some(Change::Returned));
    assert_eq!(w.observe(4_000, 7, Some(true)), None);
}

// A Wi-Fi driver busy joining may not answer within the call's budget; that
// is neither a drop nor a return.
#[test]
fn no_answer_is_neither_a_drop_nor_a_return() {
    let mut w = LinkWatch::new();
    assert_eq!(w.observe(0, 7, None), None);
    assert_eq!(w.observe(1_000, 7, Some(false)), Some(Change::Dropped));
    assert_eq!(w.observe(2_000, 7, None), None);
    assert_eq!(w.observe(3_000, 7, Some(true)), Some(Change::Returned));
}

// A stack bound to another port was built with a fresh DHCP client; a drop
// seen on the old port is not carried over to it.
#[test]
fn a_new_port_starts_as_up() {
    let mut w = LinkWatch::new();
    assert_eq!(w.observe(0, 7, Some(false)), Some(Change::Dropped));
    assert_eq!(w.observe(1_000, 9, Some(true)), None);
}

#[test]
fn the_link_is_asked_once_a_second_on_the_clock() {
    let mut w = LinkWatch::new();
    assert!(w.due(0));
    w.observe(500, 7, Some(true));
    assert!(!w.due(500 + WATCH_EVERY_MS - 1));
    assert!(w.due(500 + WATCH_EVERY_MS));
}
