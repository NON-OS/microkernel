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

//! Only the window manager may raise a layer (`state/raise_rule.rs`).

use crate::raise_rule::may_raise;

const WM: u32 = 31;

#[test]
fn the_window_manager_raises() {
    assert!(may_raise(WM, Some(WM)));
}

#[test]
fn any_other_client_is_refused() {
    for pid in [1, 2, WM - 1, WM + 1, u32::MAX] {
        assert!(!may_raise(pid, Some(WM)), "pid {pid}");
    }
}

#[test]
fn with_no_window_manager_running_nobody_raises() {
    for pid in [1, WM, u32::MAX] {
        assert!(!may_raise(pid, None), "pid {pid}");
    }
}

#[test]
fn pid_zero_never_raises() {
    assert!(!may_raise(0, Some(0)));
    assert!(!may_raise(0, Some(WM)));
}

#[test]
fn a_restarted_window_manager_is_the_one_obeyed() {
    let (old, new) = (WM, WM + 40);
    assert!(may_raise(new, Some(new)));
    assert!(!may_raise(old, Some(new)));
}
