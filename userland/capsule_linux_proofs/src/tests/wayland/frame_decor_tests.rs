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

use crate::wayland_frame::geometry::{around_at, on_screen, Around};

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

#[test]
fn a_frame_is_laid_at_its_own_rows_and_a_bare_window_at_the_content() {
    let content = (800, 600, 3200);
    let normal = around_at(800, 600, 4, false, false);
    assert_eq!(normal.laid(content), (normal.w, normal.h, normal.w * 4));
    let bare = around_at(800, 600, 4, true, false);
    assert_eq!(bare.laid(content), content, "full screen: the content's own rows");
    assert_eq!(bare.laid((800, 600, 4096)), (800, 600, 4096), "a padded stride kept");
}

#[test]
fn a_maximise_with_the_content_size_kept_needs_buffers_of_another_layout() {
    // A client acks a maximise and commits a buffer of the same size: the
    // bytes of the maximised frame fit the old buffers, but its rows do not.
    let content = (800, 600, 3200);
    let normal = around_at(800, 600, 4, false, false);
    let max = around_at(800, 600, 4, false, true);
    let full = around_at(800, 600, 4, true, false);
    let bytes = |l: (u32, u32, u32)| l.2 as usize * l.1 as usize;
    assert!(bytes(max.laid(content)) <= bytes(normal.laid(content)), "the bytes alone pass");
    assert_ne!(max.laid(content), normal.laid(content), "the layout does not");
    assert_ne!(full.laid(content), normal.laid(content), "nor full screen's");
}

#[test]
fn a_huge_frame_saturates_its_stride_rather_than_wrapping() {
    let a = Around { w: u32::MAX / 2, h: 1, cx: 1, cy: 1, max: false };
    assert_eq!(a.laid((1, 1, 4)).2, u32::MAX);
}

#[test]
fn until_the_buffer_for_a_maximise_comes_the_frame_on_screen_is_the_old_one() {
    let content = (800, 600, 3200);
    let normal = around_at(800, 600, 4, false, false);
    let max = around_at(800, 600, 4, false, true);
    // Acked, the old buffers still shown: the pointer and a redraw use the
    // frame they hold.
    assert_eq!(on_screen(max, normal, content, normal.laid(content)), normal);
    // The buffer drawn for it shown: the new frame.
    assert_eq!(on_screen(max, normal, content, max.laid(content)), max);
    // Nothing due: the two are one.
    assert_eq!(on_screen(normal, normal, content, normal.laid(content)), normal);
    // Buffers of neither: the frame asked for, as before.
    assert_eq!(on_screen(max, normal, content, (1, 1, 4)), max);
}

/// How the frame is painted, and the guest's pixels over its ground.
#[path = "frame_paint_tests.rs"]
mod paint;
