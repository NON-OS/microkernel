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
//! The menu bar names the focused app on every raise: the window manager's
//! focus notice reads as Focused, and the app it names becomes the active
//! one, or none ("Desktop") when nothing has focus or it is no launcher app's.

use super::rig::{index_of, STORE_PID, TERMINAL_PID, WINDOW};
use crate::taskbar::{focus_moved, new_taskbar_state, track_window_opened, TASKBAR_NO_ACTIVE};
use crate::wm_notice::{decode_wm_notice, WmEvent, WmNotice};
use crate::wm_notify_encode::{encode_notify, NOTIFY_LEN};

#[test]
fn the_focus_notice_reads_as_focused() {
    let mut frame = [0u8; NOTIFY_LEN];
    encode_notify(&mut frame, 3, 40, 7, 0, 0);
    let want = WmNotice { event: WmEvent::Focused, owner_pid: 40, window_id: 7 };
    assert_eq!(decode_wm_notice(&frame), Some(Some(want)));
}

/// "Processes" stayed in the bar after the Terminal came forward: only an
/// open or a close moved the name.
#[test]
fn a_raise_names_the_raised_app() {
    let (store, terminal) = (index_of(b"app.store"), index_of(b"app.terminal"));
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, 1, TERMINAL_PID, terminal);
    track_window_opened(&mut t, WINDOW, STORE_PID, store);
    assert_eq!(t.active as usize, store);
    assert!(focus_moved(&mut t, Some(terminal)), "the Terminal raised");
    assert_eq!(t.active as usize, terminal);
    assert!(!focus_moved(&mut t, Some(terminal)), "the same app again changes nothing");
    assert!(focus_moved(&mut t, Some(store)));
    assert_eq!(t.active as usize, store);
}

#[test]
fn nothing_focused_or_no_app_names_the_desktop() {
    let terminal = index_of(b"app.terminal");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, 1, TERMINAL_PID, terminal);
    assert!(focus_moved(&mut t, None));
    assert_eq!(t.active, TASKBAR_NO_ACTIVE);
    assert!(!focus_moved(&mut t, None));
    assert!(focus_moved(&mut t, Some(terminal)));
    let past = t.open.len();
    assert!(focus_moved(&mut t, Some(past)), "past the table is no app");
    assert_eq!(t.active, TASKBAR_NO_ACTIVE);
}
