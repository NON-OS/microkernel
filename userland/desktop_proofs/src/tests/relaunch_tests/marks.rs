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

//! A dock click raises a live window or opens the app afresh.

use super::rig::{index_of, WINDOW};
use crate::taskbar::{new_taskbar_state, raise_tracked, track_window_opened, Reach};

/// A Calculator window whose process crashed: no close ever reached the
/// shell, so the dock still marks it running. The click used to go to that
/// pid, then to any instance the registry named, and with neither it said
/// "did not open" and spawned nothing. Now the window is forgotten and the
/// app is left with none, so the caller opens it.
#[test]
fn a_running_mark_whose_process_is_gone_opens_the_app_afresh() {
    let calc = index_of(b"app.calculator");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, WINDOW, 300, calc);
    assert!(t.open[calc]);
    let mut asked = Vec::new();
    let raised = raise_tracked(&mut t, calc, |pid| {
        asked.push(pid);
        Reach::Gone
    });
    assert_eq!(raised, None);
    assert_eq!(asked, [300]);
    assert!(!t.open[calc], "the running mark goes with the last window");
    assert!(t.windows.is_empty());
}

/// A minimised window brought back by a dock click: its process takes the
/// focus frame, which restores it. Nothing is spawned beside it.
#[test]
fn a_minimised_window_is_raised_not_doubled() {
    let term = index_of(b"app.terminal");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, WINDOW, 101, term);
    assert_eq!(raise_tracked(&mut t, term, |_| Reach::Taken), Some(101));
    assert!(t.open[term]);
    assert_eq!(t.windows.len(), 1);
}
