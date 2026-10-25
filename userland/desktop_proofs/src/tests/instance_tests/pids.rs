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

//! Instances told apart by pid, and an app launchable off the dock.

use super::rig::index_of;
use crate::shell_apps::LAUNCHER_APPS;
use crate::taskbar::{
    new_taskbar_state, raise_tracked, track_window_closed, track_window_opened, Reach,
};

/* Every instance of an app opens a window with the id in its manifest, so
 * two terminals hold window 10 each. A second open must not evict the first,
 * and a close must take only its own. */
#[test]
fn instances_sharing_a_window_id_are_told_apart_by_pid() {
    let term = index_of(b"app.terminal");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, 10, 100, term);
    track_window_opened(&mut t, 10, 101, term);
    assert_eq!(raise_tracked(&mut t, term, |_| Reach::Taken), Some(101));
    assert_eq!(track_window_closed(&mut t, 101, 10), Some(term));
    assert!(t.open[term], "the first terminal is still up");
    assert_eq!(raise_tracked(&mut t, term, |_| Reach::Taken), Some(100));
    assert_eq!(track_window_closed(&mut t, 101, 10), None, "already gone");
    assert_eq!(track_window_closed(&mut t, 100, 10), Some(term));
    assert!(!t.open[term]);
}

/* The image viewer is in the table, so a picture opens in it and its windows
 * count as its app, but it has no dock tile: the dock's apps lead the table
 * and the viewer comes after them. */
#[test]
fn the_image_viewer_is_launchable_but_off_the_dock() {
    use crate::shell_apps::DOCK_APPS;
    let viewer = index_of(b"app.image_viewer");
    assert!(!LAUNCHER_APPS[viewer].dock);
    assert!(viewer >= DOCK_APPS);
    assert!(LAUNCHER_APPS[..DOCK_APPS].iter().all(|a| a.dock));
    assert!(LAUNCHER_APPS[DOCK_APPS..].iter().all(|a| !a.dock));
    assert_eq!(DOCK_APPS, LAUNCHER_APPS.len() - 1);
}
