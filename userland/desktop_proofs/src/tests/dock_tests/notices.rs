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

//! The shell reads the window manager's notifications.

use crate::wm_notice::{decode_wm_notice, WmEvent, WmNotice};
use crate::wm_notify_encode::{encode_notify, NOTIFY_LEN};

/// The window manager's full-screen notification as its encoder writes it,
/// decoded by the shell; the opened and closed kinds read as before, and a
/// kind the shell does not know is dropped, not misread.
#[test]
fn the_shell_reads_the_window_managers_notifications() {
    let notice = |kind: u32, x: u32| {
        let mut frame = [0u8; NOTIFY_LEN];
        encode_notify(&mut frame, kind, 40, 7, x, 0);
        decode_wm_notice(&frame)
    };
    let of = |event| Some(Some(WmNotice { event, owner_pid: 40, window_id: 7 }));
    assert_eq!(notice(0, 100), of(WmEvent::Opened));
    assert_eq!(notice(1, 100), of(WmEvent::Closed));
    assert_eq!(notice(2, 1), of(WmEvent::FullScreen(true)));
    assert_eq!(notice(2, 0), of(WmEvent::FullScreen(false)));
    assert_eq!(notice(9, 0), Some(None));
    assert_eq!(decode_wm_notice(&[0u8; 28]), None, "not a notification");
}
