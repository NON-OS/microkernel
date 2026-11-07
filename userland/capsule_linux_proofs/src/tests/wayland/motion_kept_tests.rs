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

//! A backed-up client is sent no pointer motion, but the latest one is
//! kept and goes before the next button, key or wheel, so a click lands
//! where the pointer is. A newer motion replaces it; it also goes once the
//! client has read its queue down.

#[path = "../../../../capsule_linux/src/linux/wayland/input/latest.rs"]
mod latest;

use latest::Latest;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ev {
    Move(i32),
    Click,
}

fn offer(l: &mut Latest<Ev>, e: Ev, backlogged: bool) -> Vec<Ev> {
    let motion = matches!(e, Ev::Move(_));
    l.offer(e, motion, backlogged).into_iter().flatten().collect()
}

#[test]
fn a_client_keeping_up_gets_every_event_as_it_comes() {
    let mut l = Latest::new();
    assert_eq!(offer(&mut l, Ev::Move(1), false), [Ev::Move(1)]);
    assert_eq!(offer(&mut l, Ev::Click, false), [Ev::Click]);
    assert_eq!(l.release(false), None);
}

#[test]
fn a_click_after_dropped_motion_goes_after_the_latest_position() {
    let mut l = Latest::new();
    assert_eq!(offer(&mut l, Ev::Move(1), true), []);
    assert_eq!(offer(&mut l, Ev::Move(2), true), []);
    assert_eq!(offer(&mut l, Ev::Move(3), true), []);
    assert_eq!(offer(&mut l, Ev::Click, true), [Ev::Move(3), Ev::Click]);
    // Sent once: the next click does not repeat it.
    assert_eq!(offer(&mut l, Ev::Click, true), [Ev::Click]);
}

#[test]
fn a_motion_sent_once_the_client_caught_up_replaces_the_kept_one() {
    let mut l = Latest::new();
    assert_eq!(offer(&mut l, Ev::Move(1), true), []);
    assert_eq!(offer(&mut l, Ev::Move(2), false), [Ev::Move(2)]);
    assert_eq!(offer(&mut l, Ev::Click, false), [Ev::Click]);
}

#[test]
fn the_kept_motion_goes_once_the_backlog_clears_and_not_before() {
    let mut l = Latest::new();
    assert_eq!(offer(&mut l, Ev::Move(4), true), []);
    assert_eq!(l.release(true), None);
    assert_eq!(l.release(false), Some(Ev::Move(4)));
    assert_eq!(l.release(false), None);
}
