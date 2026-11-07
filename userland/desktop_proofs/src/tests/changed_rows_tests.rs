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

//! A whole repaint of the shell is shown as the band of rows it changed
//! (capsule_desktop_shell render/changed_rows.rs, render/chrome/show.rs).
//! Shown whole, the desk and the chrome, each the size of the screen, had
//! the compositor compose the whole 1920x1080 screen once for each, even
//! when only the menu bar or the dock had changed; on a guest that was a
//! run of whole-screen frames each time the shell repainted whole.

use crate::changed_rows::{band, changed_rows};

const W: usize = 16;
const H: usize = 9;

fn frame(fill: u32) -> Vec<u32> {
    vec![fill; W * H]
}

#[test]
fn the_same_frame_changes_no_row() {
    assert_eq!(changed_rows(&frame(3), &frame(3), W), None);
    assert_eq!(band(None, 1920), None, "nothing to show");
}

#[test]
fn one_changed_pixel_is_its_row_alone() {
    for row in 0..H {
        for col in [0, W / 2, W - 1] {
            let mut next = frame(3);
            next[row * W + col] = 4;
            assert_eq!(changed_rows(&next, &frame(3), W), Some((row, row + 1)), "{row},{col}");
        }
    }
}

#[test]
fn the_band_runs_from_the_first_changed_row_to_past_the_last() {
    for first in 0..H {
        for last in first..H {
            let mut next = frame(1);
            next[first * W] = 2;
            next[last * W + W - 1] = 2;
            assert_eq!(changed_rows(&next, &frame(1), W), Some((first, last + 1)));
        }
    }
}

/// Every pixel outside the band is the same on both frames, so a present of
/// the band alone leaves the screen equal to the new frame.
#[test]
fn outside_the_band_the_frames_are_equal() {
    let mut seed = 0x9e37_79b9u32;
    for _ in 0..500 {
        let shown = frame(7);
        let mut next = shown.clone();
        for _ in 0..(seed % 5) {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            next[(seed as usize) % (W * H)] = seed;
        }
        let Some((first, end)) = changed_rows(&next, &shown, W) else {
            assert_eq!(next, shown);
            continue;
        };
        assert!(first < end && end <= H);
        let mut screen = shown.clone();
        screen[first * W..end * W].copy_from_slice(&next[first * W..end * W]);
        assert_eq!(screen, next, "the band {first}..{end} holds every change");
        assert_ne!(next[first * W..(first + 1) * W], shown[first * W..(first + 1) * W]);
        assert_ne!(next[(end - 1) * W..end * W], shown[(end - 1) * W..end * W]);
    }
}

#[test]
fn a_stride_wider_than_the_screen_is_compared_whole() {
    let rows = 4;
    let stride = W + 3;
    let shown = vec![0u32; stride * rows];
    let mut next = shown.clone();
    next[2 * stride + W + 1] = 9;
    assert_eq!(changed_rows(&next, &shown, stride), Some((2, 3)), "padding counts too");
}

#[test]
fn no_row_width_or_a_short_frame_compares_what_there_is() {
    assert_eq!(changed_rows(&frame(1), &frame(2), 0), None);
    let next = frame(1);
    assert_eq!(changed_rows(&next, &next[..W * 2], W), None, "the rows both hold agree");
    assert_eq!(changed_rows(&frame(5), &frame(1)[..W * 2], W), Some((0, 2)));
}

#[test]
fn the_band_is_the_screen_wide() {
    assert_eq!(band(Some((0, 58)), 1920), Some((0, 0, 1920, 58)), "the menu bar");
    assert_eq!(band(Some((960, 1080)), 1920), Some((0, 960, 1920, 120)), "the dock");
    assert_eq!(band(Some((5, 5)), 1920), None);
    assert_eq!(band(Some((9, 4)), 1920), None);
    assert_eq!(band(Some((0, 4)), 0), None);
}

#[test]
fn a_whole_frame_is_shown_as_its_band_and_submitted_whole_only_without_one() {
    const SHOW: &str = include_str!("../../../capsule_desktop_shell/src/render/chrome/show.rs");
    assert!(SHOW.contains(
        "    match (changed, band(rows, width)) {\n        (Changed::Whole, Some((x, y, w, h))) => Changed::Part(x, y, w, h),\n        _ => changed,\n    }"
    ));
    assert!(SHOW.contains("|| flip(ctx, ctx.desk, (DESK_Z, false), desk_at))"));
    assert!(SHOW.contains("|| flip(ctx, ctx.chrome, (chrome_z, curtain), chrome_at))"));
    const FRAME: &str = include_str!("../../../capsule_desktop_shell/src/render/chrome/frame.rs");
    assert!(FRAME.contains(
        "on_two.then(|| rows_apart(f.next_va(), f.shown_va(), words, row_words)).flatten()"
    ));
    assert!(
        FRAME.contains("let on_two = new && f.flips();"),
        "one backing has no frame to compare"
    );
}
