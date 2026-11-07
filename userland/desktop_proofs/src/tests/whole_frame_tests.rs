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

//! The compositor may read the shell's surface at any moment of a paint. A
//! chrome paint clears to transparent and draws the dock back; while it does,
//! the surface must still hold the whole previous frame, and once it returns
//! the whole new one.

use crate::whole_frame::draw_whole;

const W: usize = 16;
const H: usize = 8;
const TRANSPARENT: u32 = 0;
const DOCK: u32 = 0xFF20_2428;
const DOCK_LIT: u32 = 0xFF30_3840;

#[test]
fn the_surface_is_untouched_until_the_frame_is_whole() {
    let mut surface = vec![DOCK; W * H];
    let target = surface.as_mut_ptr() as u64;
    let mut back = Vec::new();
    let mut during = Vec::new();
    draw_whole(&mut back, target, None, W * H, W, |va| {
        let px = unsafe { core::slice::from_raw_parts_mut(va as *mut u32, W * H) };
        px.fill(TRANSPARENT);
        // The compositor reads the surface now, between the clear and the
        // dock being drawn again.
        during = unsafe { core::slice::from_raw_parts(target as *const u32, W * H) }.to_vec();
        for p in px[(H - 2) * W..].iter_mut() {
            *p = DOCK_LIT;
        }
    });
    assert!(
        during.iter().all(|&p| p == DOCK),
        "a composite during the paint read a cleared surface: the dock blinked"
    );
    assert!(surface[(H - 2) * W..].iter().all(|&p| p == DOCK_LIT));
    assert!(surface[..(H - 2) * W].iter().all(|&p| p == TRANSPARENT));
}

#[test]
fn only_changed_rows_are_written_and_the_frame_is_reused() {
    let mut surface = vec![DOCK; W * H];
    let target = surface.as_mut_ptr() as u64;
    let mut back = Vec::new();
    draw_whole(&mut back, target, None, W * H, W, |va| {
        let px = unsafe { core::slice::from_raw_parts_mut(va as *mut u32, W * H) };
        px.fill(DOCK);
    });
    let kept = back.as_ptr();
    // A clock tick: only the top row changes.
    draw_whole(&mut back, target, None, W * H, W, |va| {
        let px = unsafe { core::slice::from_raw_parts_mut(va as *mut u32, W * H) };
        px.fill(DOCK);
        px[3] = DOCK_LIT;
    });
    assert_eq!(back.as_ptr(), kept, "the frame is not allocated again");
    assert_eq!(surface[3], DOCK_LIT);
    assert!(surface[W..].iter().all(|&p| p == DOCK));
}

// A part frame the same as the one on screen is not copied or shown: the
// clock shows minutes, and its tick presented both of the shell's surfaces
// every second, so an idle desktop composed the menu bar about three times
// a second for no change. A whole frame is always shown, and a frame that
// differs from the one on screen always is.

const WORDS: usize = W * H;

fn backing(fill: u32) -> Vec<u32> {
    vec![fill; WORDS]
}

fn va(b: &mut [u32]) -> u64 {
    b.as_mut_ptr() as u64
}

/// Draw a frame of `fill` with its first row `top`.
fn frame(top: u32, fill: u32) -> impl FnOnce(u64) {
    move |at| {
        let px = unsafe { core::slice::from_raw_parts_mut(at as *mut u32, WORDS) };
        px.fill(fill);
        px[..W].fill(top);
    }
}

#[test]
fn a_part_frame_the_same_as_the_one_on_screen_is_not_shown() {
    let mut shown = backing(7);
    shown[..W].fill(1);
    let mut next = backing(9);
    let mut back = Vec::new();
    let on = va(&mut shown);
    let new = draw_whole(&mut back, va(&mut next), Some(on), WORDS, W, frame(1, 7));
    assert!(!new, "the same picture: nothing to show");
    assert_eq!(next, backing(9), "the hidden backing is left as it was");
}

#[test]
fn a_part_frame_that_differs_is_copied_and_shown() {
    let mut shown = backing(7);
    let mut next = backing(7);
    let mut back = Vec::new();
    let on = va(&mut shown);
    let new = draw_whole(&mut back, va(&mut next), Some(on), WORDS, W, frame(2, 7));
    assert!(new);
    assert_eq!(&next[..W], &[2; W], "the changed row is written");
    assert_eq!(&next[W..], &[7; WORDS - W]);
}

/// The hidden backing already holds the new picture (the frame before the
/// shown one looked the same), but the screen does not: it is shown.
#[test]
fn a_frame_the_hidden_backing_holds_but_the_screen_does_not_is_shown() {
    let mut shown = backing(7);
    shown[..W].fill(1);
    let mut next = backing(7);
    let mut back = Vec::new();
    let on = va(&mut shown);
    let before = next.clone();
    let new = draw_whole(&mut back, va(&mut next), Some(on), WORDS, W, frame(7, 7));
    assert!(new, "no row written, and still to be shown");
    assert_eq!(next, before);
}

#[test]
fn a_whole_frame_is_always_copied_and_shown() {
    let mut next = backing(5);
    let mut back = Vec::new();
    let new = draw_whole(&mut back, va(&mut next), None, WORDS, W, frame(3, 5));
    assert!(new, "the same as before, and shown all the same");
    assert_eq!(&next[..W], &[3; W]);
}

#[test]
fn a_single_surface_is_drawn_in_place_and_new_only_where_a_row_changed() {
    let mut only = backing(4);
    let mut back = Vec::new();
    let at = va(&mut only);
    assert!(!draw_whole(&mut back, at, Some(at), WORDS, W, frame(4, 4)), "nothing changed");
    assert!(draw_whole(&mut back, at, Some(at), WORDS, W, frame(6, 4)), "a row changed");
    assert_eq!(&only[..W], &[6; W]);
}

#[test]
fn the_clock_commits_only_a_frame_that_was_shown() {
    const CLOCK: &str =
        include_str!("../../../capsule_desktop_shell/src/server/runner/refresh_clock.rs");
    assert!(CLOCK.contains(
        "    for &(x, y, w, h) in paint_chrome_in(ctx, changed).owed().of(&[r]) {\n        let rid = ctx.issue_request_id();\n        let _ = push_damage_commit("
    ));
    const FRAME: &str = include_str!("../../../capsule_desktop_shell/src/render/chrome/frame.rs");
    assert!(
        FRAME.contains(
            "let (desk_shown, chrome_shown) = (Some(ctx.desk.shown_va()), Some(ctx.chrome.shown_va()));"
        ),
        "a whole frame is held to the one on screen as a part frame is"
    );
}
