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

//! The wait for a window after a click.

use super::rig::{index_of, WINDOW};
use crate::taskbar::expect::EXPECT_MS;
use crate::taskbar::{
    expect_window, new_taskbar_state, track_window_closed, track_window_opened, windows_overdue,
    Uptime,
};

/// A close, then an open: the window of the second open clears the wait the
/// click started, so nothing is said.
#[test]
fn a_window_that_comes_ends_its_wait() {
    let calc = index_of(b"app.calculator");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, WINDOW, 300, calc);
    assert_eq!(track_window_closed(&mut t, 300, WINDOW), Some(calc));
    expect_window(&mut t, calc, Uptime(1_000));
    track_window_opened(&mut t, WINDOW, 301, calc);
    assert_eq!(windows_overdue(&mut t, Uptime(1_000 + EXPECT_MS * 2)), 0);
}

/// A spawn init refused, or an instance that never opened its window: past
/// the wait the app is named once, and only once.
#[test]
fn a_window_that_never_comes_is_said_once() {
    let calc = index_of(b"app.calculator");
    let mut t = new_taskbar_state();
    expect_window(&mut t, calc, Uptime(1_000));
    assert_eq!(windows_overdue(&mut t, Uptime(1_000 + EXPECT_MS - 1)), 0);
    assert_eq!(windows_overdue(&mut t, Uptime(1_000 + EXPECT_MS)), 1u64 << calc);
    assert_eq!(windows_overdue(&mut t, Uptime(1_000 + EXPECT_MS * 3)), 0);
}

/// A second click while the first waits starts the wait again, and is one
/// wait, not two.
#[test]
fn a_second_click_restarts_the_wait() {
    let calc = index_of(b"app.calculator");
    let mut t = new_taskbar_state();
    expect_window(&mut t, calc, Uptime(1_000));
    expect_window(&mut t, calc, Uptime(9_000));
    assert_eq!(t.expecting.len(), 1);
    assert_eq!(windows_overdue(&mut t, Uptime(1_000 + EXPECT_MS)), 0);
    assert_eq!(windows_overdue(&mut t, Uptime(9_000 + EXPECT_MS)), 1u64 << calc);
}
