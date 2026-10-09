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

//! A window's frame at the shell's display scale: the title bar, buttons,
//! border and title text grow by the shell's rule, what is drawn is what the
//! hit test answers, and the drag still tells the title bar from the border.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::decorations::{
    accessory_rect, accessory_rect_at, border_at, chrome_growth_at, content_rect, content_rect_at,
    draw_frame_at, frame_rect_at, hit_test, hit_test_at, light_rect_at, margin_at, titlebar_h_at,
    titlebar_rect_at, DecorationHit, HAIRLINE, LIGHT_CLOSE, LIGHT_MAXIMIZE, LIGHT_MINIMIZE,
    TITLEBAR_H,
};

use super::chrome::quarters_for;
use super::desk::event;
use super::drag::{handle_at, DragState, PointerAction};
use crate::input::InputKind;
use crate::shell_scale;

const W: u32 = 900;
const H: u32 = 600;
const SCALES: [u32; 4] = [4, 5, 6, 8];

/// The shell's `px`, without its latch: tests run at once on threads.
fn px(v: u32, q: u32) -> u32 {
    (v * q + 2) / 4
}

#[test]
fn the_frame_takes_the_shells_scale_for_its_display() {
    for (w, h) in [(1366, 768), (1920, 1080), (2560, 1600), (3840, 2160)] {
        assert_eq!(quarters_for(w, h), shell_scale::quarters_for(w, h), "{w}x{h}");
    }
}

#[test]
fn every_measure_grows_by_the_shells_rounding() {
    for q in SCALES {
        assert_eq!(titlebar_h_at(q), px(TITLEBAR_H, q), "title bar at {q}");
        assert_eq!(margin_at(false, q), px(10, q), "shadow margin at {q}");
        assert_eq!(margin_at(true, q), 0, "a maximised window has no margin");
        assert_eq!(border_at(q), px(1, q), "border at {q}");
        let f = frame_rect_at(W, H, false, q);
        let c = content_rect_at(W, H, false, q);
        assert_eq!(c.y, f.y + titlebar_h_at(q), "content starts under the bar at {q}");
        assert_eq!(c.x, f.x + border_at(q));
        assert_eq!(c.x + c.w + border_at(q), f.x + f.w, "the right border at {q}");
    }
}

#[test]
fn one_to_one_is_the_frame_every_other_caller_already_has() {
    for (x, y) in [(20, 20), (30, 25), (50, 30), (300, 30), (300, 60), (885, 15)] {
        assert!(hit_test(W, H, false, x, y) == hit_test_at(W, H, false, x, y, 4), "{x},{y}");
    }
    assert_eq!(content_rect(W, H, false), content_rect_at(W, H, false, 4));
    assert_eq!(accessory_rect(W, H, false, 200), accessory_rect_at(W, H, false, 200, 4));
}

#[test]
fn each_button_answers_where_it_is_drawn_at_every_scale() {
    let hits =
        [DecorationHit::CloseButton, DecorationHit::MinimizeButton, DecorationHit::MaximizeButton];
    for q in SCALES {
        let bar = titlebar_rect_at(W, H, false, q);
        let mut last_right = bar.x;
        for (i, want) in hits.iter().enumerate() {
            let r = light_rect_at(i as u32, W, H, false, q);
            assert_eq!(r.w, px(12, q), "button {i} is the scaled size at {q}");
            assert!(r.x >= last_right, "buttons do not overlap at {q}");
            last_right = r.x + r.w;
            assert!(r.y >= bar.y && r.y + r.h <= bar.y + bar.h, "button {i} sits in the bar");
            let (cx, cy) = (r.x + r.w / 2, r.y + r.h / 2);
            assert!(hit_test_at(W, H, false, cx, cy, q) == *want, "button {i} centre at {q}");
            // The target reaches past the drawn circle by the scaled pad.
            let pad = px(4, q);
            assert!(
                hit_test_at(W, H, false, r.x + r.w + pad - 1, cy, q) != DecorationHit::Titlebar
            );
        }
        // Past the buttons the bar moves the window, below it is content.
        let mid = bar.x + bar.w / 2;
        assert!(hit_test_at(W, H, false, mid, bar.y + bar.h - 1, q) == DecorationHit::Titlebar);
        assert!(hit_test_at(W, H, false, mid, bar.y + bar.h, q) == DecorationHit::None);
    }
}

#[test]
fn a_window_grows_by_the_frame_so_its_content_keeps_its_size() {
    for q in SCALES {
        let (gw, gh) = chrome_growth_at(q);
        let asked = content_rect(760, 520, false);
        let got = content_rect_at(760 + gw, 520 + gh, false, q);
        assert_eq!((got.w, got.h), (asked.w, asked.h), "content at {q}");
    }
    assert_eq!(chrome_growth_at(4), (0, 0));
}

#[test]
fn the_title_bar_drags_and_the_border_below_it_resizes_at_every_scale() {
    for q in SCALES {
        let bar = titlebar_rect_at(W, H, false, q);
        let bottom = (bar.y + bar.h) as i32;
        // The right end of the bar's lowest row moves the window.
        let mut s = DragState::new();
        let at = event(InputKind::ButtonDown, W as i32 - px(15, q) as i32, bottom - 1);
        handle_at(&mut s, (W, H), (100, 100), false, &at, q);
        assert!(s.active, "the bar's last row moves the window at {q}");
        let to = event(InputKind::PointerAbs, W as i32 - px(15, q) as i32 + 40, bottom + 29);
        let a = handle_at(&mut s, (W, H), (100, 100), false, &to, q);
        assert!(matches!(a, PointerAction::MoveTo(140, 130)), "moves by the delta at {q}");

        // Just below it the right border resizes, by the pointer's travel.
        let mut s = DragState::new();
        let at = event(InputKind::ButtonDown, W as i32 - px(15, q) as i32, bottom + 1);
        handle_at(&mut s, (W, H), (100, 100), false, &at, q);
        assert!(!s.active, "below the bar the border resizes at {q}");
        let to = event(InputKind::PointerAbs, 1000, 300);
        handle_at(&mut s, (W, H), (100, 100), false, &to, q);
        let up = event(InputKind::ButtonUp, 1000, 300);
        let done = handle_at(&mut s, (W, H), (100, 100), false, &up, q);
        let want = W + 1000 - (W - px(15, q));
        assert!(matches!(done, PointerAction::ResizeTo(w, 600) if w == want), "resize at {q}");

        // A press on the close button neither drags nor resizes.
        let close = light_rect_at(0, W, H, false, q);
        let mut s = DragState::new();
        let at = event(InputKind::ButtonDown, (close.x + 1) as i32, (close.y + 1) as i32);
        handle_at(&mut s, (W, H), (100, 100), false, &at, q);
        assert!(!s.active, "the close button is not a drag at {q}");
    }
}

/// Paints the frame and reads back the pixels the hit test talks about.
fn paint(q: u32, title: &[u8]) -> Vec<u32> {
    let mut px = vec![0u32; (W * H) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: W, width: W, height: H };
    draw_frame_at(&mut fb, false, title, false, 0, q);
    px
}

#[test]
fn what_is_drawn_is_where_the_geometry_says() {
    let lights = [LIGHT_CLOSE, LIGHT_MINIMIZE, LIGHT_MAXIMIZE];
    for q in SCALES {
        let px = paint(q, b"");
        for (i, argb) in lights.iter().enumerate() {
            let r = light_rect_at(i as u32, W, H, false, q);
            let centre = px[((r.y + r.h / 2) * W + r.x + r.w / 2) as usize];
            assert_eq!(centre, *argb, "button {i} painted at its centre at {q}");
        }
        // The hairline under the bar is the content's top edge.
        let c = content_rect_at(W, H, false, q);
        assert_eq!(px[(c.y * W + W / 2) as usize], HAIRLINE, "the bar's hairline at {q}");
    }
}

/// Rows of the title bar the title's ink reaches.
fn title_rows(q: u32) -> (u32, u32) {
    let plain = paint(q, b"");
    let titled = paint(q, b"Files");
    let bar = titlebar_rect_at(W, H, false, q);
    let (mut top, mut bottom) = (u32::MAX, 0);
    for y in 0..H {
        for x in W / 3..W * 2 / 3 {
            let i = (y * W + x) as usize;
            if plain[i] != titled[i] {
                assert!(y >= bar.y && y < bar.y + bar.h, "title ink at row {y} outside the bar");
                top = top.min(y);
                bottom = bottom.max(y);
            }
        }
    }
    assert!(top <= bottom, "the title is drawn at {q}");
    (top, bottom)
}

#[test]
fn the_title_grows_with_the_bar_and_stays_inside_it() {
    let (t1, b1) = title_rows(4);
    let (t2, b2) = title_rows(8);
    let (h1, h2) = (b1 - t1 + 1, b2 - t2 + 1);
    assert!(h2 * 10 >= h1 * 17 && h2 * 10 <= h1 * 23, "type at 2x is about twice: {h1} then {h2}");
    for q in SCALES {
        title_rows(q);
    }
}
