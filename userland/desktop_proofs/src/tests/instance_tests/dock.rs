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

//! The dock and the menu bar follow an app's newest window.

use super::rig::index_of;
use crate::taskbar::{
    new_taskbar_state, raise_tracked, track_window_closed, track_window_opened, Reach,
};

#[test]
fn a_dock_click_raises_the_newest_window_of_that_app() {
    let term = index_of(b"app.terminal");
    let files = index_of(b"app.file_manager");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, 10, 100, term);
    track_window_opened(&mut t, 20, 200, files);
    track_window_opened(&mut t, 11, 101, term);
    assert_eq!(raise_tracked(&mut t, term, |_| Reach::Taken), Some(101));
    assert_eq!(raise_tracked(&mut t, files, |_| Reach::Taken), Some(200));
    track_window_closed(&mut t, 200, 20);
    assert_eq!(raise_tracked(&mut t, files, |_| Reach::Taken), None);
}

#[test]
fn the_menubar_follows_the_app_whose_window_opened_last() {
    let term = index_of(b"app.terminal");
    let files = index_of(b"app.file_manager");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, 10, 100, term);
    track_window_opened(&mut t, 20, 200, files);
    assert_eq!(t.active as usize, files);
    track_window_closed(&mut t, 200, 20);
    assert_eq!(t.active as usize, term);
}

#[test]
fn an_opened_event_repeated_for_one_window_counts_once() {
    let term = index_of(b"app.terminal");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, 10, 100, term);
    track_window_opened(&mut t, 10, 100, term);
    track_window_closed(&mut t, 100, 10);
    assert!(!t.open[term]);
}
