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

//! The NØNOS frame around a guest's window: the content sits inside it at
//! every scale, nothing of the desktop shows through where the guest drew
//! nothing, and the guest's own pixels land where its content is.

use crate::wayland_frame::geometry::{around_at, Around};

#[test]
fn the_frame_holds_the_content_and_grows_with_the_scale() {
    for q in [4, 5, 6, 8] {
        let a = around_at(800, 600, q, false, false);
        assert!(a.framed() && a.cx > 0 && a.cy > a.cx, "a title bar above, q {q}");
        assert!(a.w >= 800 + 2 * a.cx && a.h >= 600 + a.cy, "q {q}");
    }
    assert!(around_at(800, 600, 8, false, false).cy > around_at(800, 600, 4, false, false).cy);
}

#[test]
fn a_window_that_fills_the_display_has_no_frame() {
    assert_eq!(around_at(1920, 1022, 4, true, false), Around::bare(1920, 1022));
}

#[test]
fn the_pointer_is_on_the_content_only_inside_it() {
    let a = around_at(100, 50, 4, false, false);
    assert_eq!(a.on_content(a.cx as i32, a.cy as i32, 100, 50), Some((0, 0)));
    assert_eq!(a.on_content(a.cx as i32 + 99, a.cy as i32 + 49, 100, 50), Some((99, 49)));
    assert_eq!(a.on_content(a.cx as i32 + 100, a.cy as i32, 100, 50), None);
    assert_eq!(a.on_content(a.cx as i32 - 1, a.cy as i32, 100, 50), None, "the border");
    assert_eq!(a.on_content(a.cx as i32, a.cy as i32 - 1, 100, 50), None, "the title bar");
}

#[test]
fn a_maximised_window_keeps_its_frame_and_only_full_screen_drops_it() {
    use crate::fit::Mode;
    assert!(!Mode::Maximized.bare() && Mode::Maximized.fills(), "the dock still hides");
    assert!(Mode::FullScreen.bare() && !Mode::Normal.bare());
    let framed = around_at(800, 600, 4, Mode::Maximized.bare(), true);
    let normal = around_at(800, 600, 4, false, false);
    assert!(framed.max && framed.w < normal.w, "no margin round a maximised frame");
    assert!(framed.framed() && framed.w > 800 && framed.h > 600);
}

/// How the frame is painted, and the guest's pixels over its ground.
#[path = "frame_paint_tests.rs"]
mod paint;
