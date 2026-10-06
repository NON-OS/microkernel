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

//! window_open for a window the table still holds (window/reopen.rs).

use crate::focus::topmost_hit_at;
use crate::geometry::Rect;
use crate::window::reopen::reopen;
use crate::window::{Kind, Visibility, Window};

fn left_maximised_and_minimised() -> Window {
    Window {
        owner_pid: 40,
        window_id: 1,
        rect: Rect { x: 0, y: 46, width: 1280, height: 594 },
        kind: Kind::Normal,
        visibility: Visibility::Minimized,
        z: 3,
        in_use: true,
        full_screen: false,
    }
}

/// The app opens its window again at the size its new surface has. The answer
/// was the old, maximised rect, which the app took as its surface's size.
#[test]
fn a_reopened_window_takes_the_size_its_client_asks_for_now() {
    let mut w = left_maximised_and_minimised();
    reopen(&mut w, Kind::Normal, Rect { x: 300, y: 200, width: 640, height: 480 }, 1280, 720);
    assert_eq!((w.rect.x, w.rect.y, w.rect.width, w.rect.height), (0, 46, 640, 480));
}

/// It was left minimised: drawn again, it must take presses again.
#[test]
fn a_reopened_window_is_on_screen_and_hit() {
    let mut table = crate::window::WindowTable::new();
    let mut w = left_maximised_and_minimised();
    reopen(&mut w, Kind::Normal, Rect { x: 0, y: 0, width: 640, height: 480 }, 1280, 720);
    table.insert(w).expect("room");
    assert!(w.visibility == Visibility::Visible);
    assert_eq!(topmost_hit_at(&table, 100, 100, 0).map(|h| h.owner_pid), Some(40));
}

#[test]
fn a_reopened_window_stays_on_the_display() {
    let mut w = left_maximised_and_minimised();
    w.rect.x = 1000;
    reopen(&mut w, Kind::Normal, Rect { x: 0, y: 0, width: 640, height: 480 }, 1280, 720);
    assert!(w.rect.x + w.rect.width <= 1280);
}
