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

//! The green button's full screen as the window manager keeps it
//! (window/full_screen.rs), and the notification that tells the desktop
//! shell to hide its dock (protocol/notify.rs).

use crate::geometry::Rect;
use crate::protocol::{encode_notify, NOTIFY_KIND_FULL_SCREEN, NOTIFY_LEN};
use crate::window::full_screen::{
    covers_dock, covers_dock_at, maximize, MAXIMIZE_FLAG_FULL_SCREEN,
};
use crate::window::reopen::reopen;
use crate::window::{Kind, Visibility, Window, WindowTable};

fn window() -> Window {
    Window {
        owner_pid: 40,
        window_id: 7,
        rect: Rect { x: 200, y: 120, width: 640, height: 480 },
        kind: Kind::Normal,
        visibility: Visibility::Visible,
        z: 3,
        in_use: true,
        full_screen: false,
    }
}

/// The rect app_skeleton asks for: the whole width, from the foot of the bar
/// to the bottom edge.
fn full(width: u32, height: u32, bar: u32) -> Rect {
    Rect { x: 0, y: bar, width, height: height - bar }
}

/// At every display and bar the shell draws, the full-screen rect is kept as
/// asked: it reaches the bottom edge, where the dock was.
#[test]
fn a_full_screen_window_reaches_the_bottom_edge_at_every_scale() {
    for (w, h, bar) in [(1280, 720, 46), (1366, 768, 46), (1920, 1080, 58), (2560, 1600, 69)] {
        let mut win = window();
        maximize(&mut win, full(w, h, bar), MAXIMIZE_FLAG_FULL_SCREEN, w, h);
        let r = win.rect;
        assert_eq!((r.x, r.y, r.width), (0, bar, w), "{w}x{h}");
        assert_eq!(r.y + r.height, h, "{w}x{h}: down to the bottom edge");
        assert!(covers_dock(&win), "{w}x{h}");
    }
}

/// Green again: the saved rect, sent with no flag, gives the dock back.
#[test]
fn restoring_the_saved_rect_ends_full_screen() {
    let mut win = window();
    maximize(&mut win, full(1280, 720, 46), MAXIMIZE_FLAG_FULL_SCREEN, 1280, 720);
    maximize(&mut win, Rect { x: 200, y: 120, width: 640, height: 480 }, 0, 1280, 720);
    assert!(!covers_dock(&win));
    assert_eq!((win.rect.x, win.rect.y), (200, 120));
}

/// A client that sends the old zero padding maximises as before and never
/// hides the dock, even with a rect that reaches the bottom.
#[test]
fn an_old_client_never_hides_the_dock() {
    let mut win = window();
    maximize(&mut win, full(1280, 720, 46), 0, 1280, 720);
    assert!(!covers_dock(&win));
}

/// Minimised it covers nothing; brought back from the dock it covers again.
#[test]
fn a_minimised_full_screen_window_covers_nothing_until_restored() {
    let mut table = WindowTable::new();
    let mut win = window();
    maximize(&mut win, full(1280, 720, 46), MAXIMIZE_FLAG_FULL_SCREEN, 1280, 720);
    table.insert(win).expect("room");
    assert!(covers_dock_at(&table, 40, 7));
    table.find_mut(40, 7).expect("open").visibility = Visibility::Minimized;
    assert!(!covers_dock_at(&table, 40, 7));
    table.find_mut(40, 7).expect("open").visibility = Visibility::Visible;
    assert!(covers_dock_at(&table, 40, 7));
    // Closed (or its process gone): nothing covers.
    let _ = table.remove(40, 7);
    assert!(!covers_dock_at(&table, 40, 7));
}

/// Opened again at the size its new surface has, it is not full screen.
#[test]
fn a_reopened_window_is_not_full_screen() {
    let mut win = window();
    maximize(&mut win, full(1280, 720, 46), MAXIMIZE_FLAG_FULL_SCREEN, 1280, 720);
    reopen(&mut win, Kind::Normal, Rect { x: 0, y: 0, width: 640, height: 480 }, 1280, 720);
    assert!(!covers_dock(&win));
}

/// The notification keeps the envelope every subscriber decodes, version 1,
/// with kind 2 and the state in `x`.
#[test]
fn the_full_screen_notification_keeps_the_envelope() {
    let mut frame = [0u8; NOTIFY_LEN];
    encode_notify(&mut frame, NOTIFY_KIND_FULL_SCREEN, 40, 7, 1, 0);
    assert_eq!(NOTIFY_LEN, 28);
    assert_eq!(&frame[0..4], &0x4E57_4D56u32.to_le_bytes());
    assert_eq!(u16::from_le_bytes([frame[4], frame[5]]), 1);
    let word = |at: usize| u32::from_le_bytes(frame[at..at + 4].try_into().expect("4"));
    assert_eq!((word(8), word(12), word(16), word(20)), (2, 40, 7, 1));
}
