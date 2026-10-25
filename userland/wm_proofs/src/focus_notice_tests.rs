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
//! Subscribers hear that focus moved whenever it did, by whatever path:
//! the rule, and the run loop asking it after every request and every
//! sweep of dead processes.

use crate::focus::model::FocusedRef;
use crate::focus::notice::focus_notice;

const RUN: &str = include_str!("../../capsule_wm/src/server/runner/run.rs");

fn at(owner_pid: u32, window_id: u32) -> Option<FocusedRef> {
    Some(FocusedRef { owner_pid, window_id })
}

#[test]
fn a_move_is_told_and_no_move_is_not() {
    assert_eq!(focus_notice(at(40, 7), at(40, 7)), None);
    assert_eq!(focus_notice(None, None), None);
    assert_eq!(focus_notice(at(40, 7), at(41, 9)), Some((41, 9)));
    assert_eq!(focus_notice(at(40, 7), at(40, 8)), Some((40, 8)), "another window");
    assert_eq!(focus_notice(None, at(41, 9)), Some((41, 9)));
    assert_eq!(focus_notice(at(40, 7), None), Some((0, 0)), "nothing has focus");
}

#[test]
fn the_loop_tells_after_every_request_and_sweep() {
    for step in ["sweep_dead(&mut ctx);", "dispatch(&mut ctx, sender_pid, req, body, &mut tx);"] {
        let at = RUN.find(step).expect(step);
        let before = &RUN[..at];
        let after = &RUN[at + step.len()..];
        let was = before.rfind("let was = ctx.focus.current();").expect("read before");
        assert!(at - was < 80, "{step}: read just before");
        assert!(after.trim_start().starts_with("tell_if_moved(&mut ctx, was);"), "{step}");
    }
}
