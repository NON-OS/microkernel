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

//! An app runs while any of its windows is open.

use super::rig::index_of;
use crate::taskbar::{
    new_taskbar_state, raise_tracked, track_window_closed, track_window_opened, Reach,
    TASKBAR_NO_ACTIVE,
};

#[test]
fn a_second_window_closing_leaves_the_app_running() {
    let term = index_of(b"app.terminal");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, 10, 100, term);
    track_window_opened(&mut t, 11, 101, term);
    assert_eq!(track_window_closed(&mut t, 101, 11), Some(term));
    assert!(t.open[term]);
    assert_eq!(t.active as usize, term);
    assert_eq!(track_window_closed(&mut t, 100, 10), Some(term));
    assert!(!t.open[term]);
    assert_eq!(t.active, TASKBAR_NO_ACTIVE);
}

#[test]
fn closing_the_first_window_keeps_the_later_instance() {
    let term = index_of(b"app.terminal");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, 10, 100, term);
    track_window_opened(&mut t, 11, 101, term);
    track_window_closed(&mut t, 100, 10);
    assert!(t.open[term]);
    assert_eq!(raise_tracked(&mut t, term, |_| Reach::Taken), Some(101));
}

#[test]
fn a_close_the_shell_never_saw_open_changes_nothing() {
    let term = index_of(b"app.terminal");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, 10, 100, term);
    assert_eq!(track_window_closed(&mut t, 999, 99), None);
    assert!(t.open[term]);
}
