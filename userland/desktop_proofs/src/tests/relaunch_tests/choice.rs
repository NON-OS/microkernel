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

//! Which window a click picks.

use super::rig::{index_of, WINDOW};
use crate::taskbar::{new_taskbar_state, raise_tracked, track_window_opened, Reach};

/// The newest window's process is gone and an older one is up: the older
/// one is raised, and only the gone one is forgotten.
#[test]
fn a_gone_newest_window_falls_back_to_the_next_one_alive() {
    let term = index_of(b"app.terminal");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, WINDOW, 100, term);
    track_window_opened(&mut t, WINDOW, 101, term);
    let raised =
        raise_tracked(&mut t, term, |pid| if pid == 101 { Reach::Gone } else { Reach::Taken });
    assert_eq!(raised, Some(100));
    assert_eq!(t.windows.len(), 1);
    assert_eq!(t.windows[0].pid, 100);
    assert!(t.open[term]);
}

/// A process whose inbox is full is alive, with its window: it is not
/// forgotten, and no second window is opened over it.
#[test]
fn a_busy_window_is_kept() {
    let files = index_of(b"app.file_manager");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, WINDOW, 200, files);
    assert_eq!(raise_tracked(&mut t, files, |_| Reach::Busy), Some(200));
    assert!(t.open[files]);
}

/// Another app's windows are never asked or forgotten.
#[test]
fn only_the_clicked_apps_windows_are_asked() {
    let term = index_of(b"app.terminal");
    let files = index_of(b"app.file_manager");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, WINDOW, 200, files);
    assert_eq!(raise_tracked(&mut t, term, |_| Reach::Gone), None);
    assert!(t.open[files]);
    assert_eq!(t.windows.len(), 1);
}
