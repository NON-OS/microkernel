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

//! A guest window's commit damages where the window is on screen. It named
//! the screen's top-left corner, so a centred Qwen window showed new text
//! only where it overlapped that corner, and kept stale pixels elsewhere.

use crate::window_damage::window_damage;

#[test]
fn a_centred_window_damages_where_it_is() {
    // An 880 by 700 window the window manager placed centred on 1920x1080.
    let at = Some((520, 238));
    assert_eq!(window_damage(at, 880, 700), Some((520, 238, 880, 700)));
}

#[test]
fn every_pixel_of_the_window_is_in_its_damage() {
    let (at, w, h) = ((520u32, 238u32), 880u32, 700u32);
    let (x, y, dw, dh) = window_damage(Some(at), w, h).expect("placed");
    let corners =
        [(at.0, at.1), (at.0 + w - 1, at.1), (at.0, at.1 + h - 1), (at.0 + w - 1, at.1 + h - 1)];
    for (px, py) in corners {
        assert!(px >= x && px < x + dw && py >= y && py < y + dh, "({px}, {py}) left out");
    }
}

#[test]
fn an_unplaced_or_empty_window_commits_nothing() {
    assert_eq!(window_damage(None, 880, 700), None);
    assert_eq!(window_damage(Some((0, 48)), 0, 700), None);
}
