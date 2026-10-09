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

//! The compositor's late answers and lost calls (state/restack.rs) and the
//! order a restack lifts the stack in (z_order/bottom_up.rs).

use crate::geometry::Rect;
use crate::restack::{Outcome, Restack, FIRST_WAIT_MS, MAX_WAIT_MS};
use crate::window::{Kind, Visibility, Window, WindowTable};
use crate::z_order::{bottom_up, raise, ZStack};

/// "ipc.call unanswered compositor caller=wm": the compositor took the
/// focus_set and answered after the 16 ms. It acts on it in order, so it is
/// not sent again.
#[test]
fn a_late_answer_is_a_delivered_call() {
    let mut r = Restack::new();
    r.note(Outcome::Late, 1000);
    r.note(Outcome::Answered, 1001);
    assert!(!r.owed());
    assert!(!r.due(1_000_000));
}

#[test]
fn a_lost_call_owes_a_restack_after_the_first_wait() {
    let mut r = Restack::new();
    r.note(Outcome::Lost, 1000);
    assert!(r.owed());
    assert!(!r.due(1000 + FIRST_WAIT_MS - 1), "not at once: the compositor just refused");
    assert!(r.due(1000 + FIRST_WAIT_MS));
    // More losses before it is due do not put it off.
    r.note(Outcome::Lost, 1020);
    assert!(r.due(1000 + FIRST_WAIT_MS));
}

#[test]
fn a_restack_that_is_lost_too_waits_twice_as_long_up_to_the_cap_and_never_gives_up() {
    let mut r = Restack::new();
    r.note(Outcome::Lost, 0);
    let mut now = FIRST_WAIT_MS;
    let mut wait = FIRST_WAIT_MS;
    for _ in 0..20 {
        assert!(r.due(now));
        r.retry(now);
        wait = (wait * 2).min(MAX_WAIT_MS);
        assert!(!r.due(now + wait - 1));
        now += wait;
    }
    assert_eq!(wait, MAX_WAIT_MS);
    assert!(r.due(now), "still asking after twenty losses");
    r.done();
    assert!(!r.owed());
    r.note(Outcome::Lost, now);
    assert!(r.due(now + FIRST_WAIT_MS), "a restack that went through starts the wait over");
}

fn open(windows: &mut WindowTable, z: &mut ZStack, who: (u32, u32), kind: Kind) {
    let window = Window {
        owner_pid: who.0,
        window_id: who.1,
        rect: Rect { x: 0, y: 0, width: 10, height: 10 },
        kind,
        visibility: Visibility::Visible,
        z: z.allocate(),
        in_use: true,
        full_screen: false,
    };
    windows.insert(window).expect("room");
}

fn order(windows: &WindowTable) -> Vec<u32> {
    let mut out = Vec::new();
    bottom_up(windows, |pid| {
        out.push(pid);
        true
    });
    out
}

#[test]
fn a_restack_lifts_each_process_once_bottom_first_at_its_highest_window() {
    let (mut w, mut z) = (WindowTable::new(), ZStack::new());
    open(&mut w, &mut z, (5, 1), Kind::Popup);
    open(&mut w, &mut z, (40, 1), Kind::Normal);
    open(&mut w, &mut z, (41, 1), Kind::Normal);
    open(&mut w, &mut z, (40, 2), Kind::Dialog);
    open(&mut w, &mut z, (42, 1), Kind::Normal);
    assert_eq!(order(&w), vec![5, 41, 40, 42]);
    raise(&mut w, &mut z, 41, 1).expect("open");
    assert_eq!(order(&w), vec![5, 40, 42, 41]);
    w.find_mut(41, 1).expect("open").visibility = Visibility::Minimized;
    assert_eq!(order(&w), vec![5, 40, 42], "a minimised window has no layer to lift");
}

#[test]
fn a_restack_stops_at_the_first_lost_call() {
    let (mut w, mut z) = (WindowTable::new(), ZStack::new());
    for pid in 40..45 {
        open(&mut w, &mut z, (pid, 1), Kind::Normal);
    }
    let mut calls = 0;
    bottom_up(&w, |_| {
        calls += 1;
        calls < 2
    });
    assert_eq!(calls, 2, "a compositor that is gone costs one more call, not five");
}
