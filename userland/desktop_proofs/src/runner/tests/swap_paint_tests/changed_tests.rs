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

//! A whole frame an app asks for in a window whose size and state did not
//! change is presented as the rows it changed from the buffer on screen
//! (runner/changed_band.rs, paint_frame.rs paint_changed): the screen still
//! equals the app after every frame, whatever the compositor holds or
//! refuses, and a frame that changed two rows composes those rows and the
//! ones between, not the window.

use super::new_path::New;
use super::rig::{content, App, H, W};
use nonos_app_skeleton::runner::changed_band::{band, changed_rows, present_rows, Shown};
use nonos_app_skeleton::runner::changed_runs::{runs_in, GAP_ROWS};
use nonos_app_skeleton::swap::{Damage, MAX_PARTS};
use nonos_toolkit::decorations::Rect;

fn xorshift(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

fn fresh(app: &App) -> Vec<u32> {
    let mut px = vec![0; (W * H) as usize];
    app.draw_whole(&mut px);
    px
}

/// Some cells changed, or none, inside the content.
fn change(s: &mut u64, app: &mut App) {
    let c = content();
    let r = xorshift(s);
    if r.is_multiple_of(5) {
        return;
    }
    let (x, y) = ((r % c.w as u64) as u32, ((r >> 16) % c.h as u64) as u32);
    let (w, h) = (1 + ((r >> 32) % 20) as u32, 1 + ((r >> 48) % 12) as u32);
    let cut = Rect { x, y, w: w.min(c.w - x), h: h.min(c.h - y) };
    app.change(cut, 0xFF00_0000 | (xorshift(s) as u32 & 0x00FF_FFFF));
}

#[test]
fn the_screen_equals_the_app_after_every_frame_whatever_is_held_or_refused() {
    for seed in 1..=40u64 {
        let mut s = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
        let (mut app, mut new) = (App::new(), New::blank());
        assert!(new.whole(&app), "the first frame");
        for step in 0..150 {
            new.hold = (xorshift(&mut s) % 3) as u32;
            new.refuse_next = xorshift(&mut s).is_multiple_of(17);
            change(&mut s, &mut app);
            let r = xorshift(&mut s) % 4;
            // A frame after one not shown is painted again as ensure_primed
            // paints it, by its changed rows.
            let shown = match (new.primed, r) {
                (false, _) => new.changed(&app),
                (true, 0) => {
                    let c = content();
                    new.partial(&app, Rect { x: 0, y: 0, w: c.w, h: c.h }, false)
                }
                (true, _) => new.changed(&app),
            };
            new.primed = shown;
            new.tick();
            if shown {
                assert!(new.screen == fresh(&app), "seed {seed} step {step}: screen not the app");
                assert!(new.bufs[new.shown] == fresh(&app), "seed {seed} step {step}: shown");
            }
        }
    }
}

#[test]
fn rows_changed_apart_compose_those_rows_and_not_the_window() {
    let (mut app, mut new) = (App::new(), New::blank());
    assert!(new.whole(&app));
    new.tick();
    // Content rows 10, and 30 and 31: 20 rows apart, more than GAP_ROWS.
    app.change(Rect { x: 3, y: 10, w: 5, h: 1 }, 0xFF11_2233);
    app.change(Rect { x: 40, y: 30, w: 9, h: 2 }, 0xFF44_5566);
    let before = new.composed;
    assert!(new.changed(&app));
    assert_eq!(new.composed - before, u64::from(W) * 3, "the three rows, not the 22 between");
    assert!(new.screen == fresh(&app));
    // Rows a few apart are one band.
    new.tick();
    app.change(Rect { x: 3, y: 40, w: 5, h: 1 }, 0xFF11_2234);
    app.change(Rect { x: 40, y: 44, w: 9, h: 1 }, 0xFF44_5567);
    let before = new.composed;
    assert!(new.changed(&app));
    assert_eq!(new.composed - before, u64::from(W) * 5, "rows 40 to 44 of the content");
    assert!(new.screen == fresh(&app));
}

#[test]
fn a_frame_that_changed_nothing_is_not_presented() {
    let (app, mut new) = (App::new(), New::blank());
    assert!(new.whole(&app));
    new.tick();
    let (shown, before) = (new.shown, new.composed);
    assert!(new.changed(&app), "given back, nothing refused");
    assert_eq!(new.shown, shown);
    assert_eq!(new.composed, before);
    assert!(new.screen == fresh(&app));
    // The buffer given back is drawn whole the next time it is taken.
    let mut app = app;
    app.change(Rect { x: 0, y: 0, w: 1, h: 1 }, 0xFF00_00FF);
    assert!(new.changed(&app));
    assert!(new.screen == fresh(&app));
}

#[test]
fn rows_are_compared_from_both_ends_and_short_frames_are_refused() {
    let a = vec![1u32; 4 * 6];
    let mut b = a.clone();
    assert_eq!(changed_rows(&a, &b, 4, 6), None, "no row differs");
    b[4 * 2 + 1] = 9;
    b[4 * 4 + 3] = 9;
    assert_eq!(changed_rows(&a, &b, 4, 6), Some((2, 5)));
    assert_eq!(changed_rows(&a, &b, 4, 2), None, "rows past `rows` are not read");
    assert_eq!(changed_rows(&a, &b[..20], 4, 6), None, "too short for six rows");
    assert_eq!(changed_rows(&a, &b, 0, 6), None);
    assert_eq!(changed_rows(&a, &b, usize::MAX, 2), None, "the length wraps");
    let longer = [a.clone(), vec![7; 8]].concat();
    assert_eq!(changed_rows(&longer, &b, 4, 6), Some((2, 5)), "words past the frame unread");
}

#[test]
fn the_band_is_the_rows_the_window_wide() {
    assert_eq!(band(Some((2, 5)), 160), Some(Damage { x: 0, y: 2, width: 160, height: 3 }));
    assert_eq!(band(None, 160), None);
    assert_eq!(band(Some((5, 5)), 160), None);
    assert_eq!(band(Some((6, 5)), 160), None);
    assert_eq!(band(Some((0, 1)), 0), None);
}

/// Keys come faster than the compositor composes: each frame finds the
/// buffer it needs still held, is not shown, and the next is painted again.
/// Painted again by its changed rows, the frame that catches up composes the
/// rows changed since the screen's frame, not the window, and the screen is
/// the app; presented whole, as before, every one composed the window.
#[test]
fn a_frame_painted_again_after_one_not_shown_composes_its_changed_rows() {
    let (mut app, mut new) = (App::new(), New::blank());
    assert!(new.whole(&app));
    new.tick();
    let c = content();
    let mut caught_up = 0;
    for key in 0..12u32 {
        // The compositor is still reading the last frame shown.
        new.hold = 3;
        // A key's cell on the input row, clear of the frame's corners.
        let cell = Rect { x: 20 + key, y: c.h / 2, w: 1, h: 1 };
        app.change(cell, 0xFF00_0000 | (key + 1));
        let shown = new.primed && new.partial(&app, cell, true);
        new.primed = shown;
        if shown {
            continue;
        }
        for _ in 0..4 {
            new.tick();
        }
        let before = new.composed;
        assert!(new.changed(&app), "key {key}: a buffer is free again");
        new.primed = true;
        assert_eq!(new.composed - before, u64::from(W), "key {key}: the one row the keys changed");
        assert!(new.screen == fresh(&app), "key {key}: the screen is the app");
        caught_up += 1;
    }
    assert!(caught_up >= 5, "most keys found their buffer held: {caught_up}");
}

#[test]
fn a_frame_it_cannot_compare_is_presented_whole() {
    let whole = Shown::Band(Damage { x: 0, y: 0, width: 4, height: 6 });
    let a = vec![1u32; 4 * 6];
    let mut b = a.clone();
    assert_eq!(present_rows(&a, &a, (4, 6), (4, 6)), Shown::Nothing, "unchanged: nothing");
    b[4 * 3] = 2;
    assert_eq!(
        present_rows(&b, &a, (4, 6), (4, 6)),
        Shown::Band(Damage { x: 0, y: 3, width: 4, height: 1 })
    );
    assert_eq!(present_rows(&a, &a[..20], (4, 6), (4, 6)), whole, "the shown one too short");
    assert_eq!(present_rows(&a[..20], &a, (4, 6), (4, 6)), whole, "the drawn one too short");
    assert_eq!(present_rows(&a, &a, (0, 6), (4, 6)), whole);
    assert_eq!(present_rows(&a, &a, (usize::MAX, 2), (4, 6)), whole, "the length wraps");
}

/// A frame of `rows` rows 4 pixels wide whose rows in `changed` differ.
fn frames(rows: usize, changed: &[usize]) -> (Vec<u32>, Vec<u32>) {
    let shown = vec![1u32; 4 * rows];
    let mut next = shown.clone();
    for &r in changed {
        next[4 * r + 2] = 7;
    }
    (next, shown)
}

fn shown_rows(next: &[u32], shown: &[u32], rows: usize) -> Shown {
    present_rows(next, shown, (4, rows), (4, rows as u32))
}

#[test]
fn rows_changed_far_apart_are_two_boxes_inside_their_band() {
    let (next, shown) = frames(200, &[10, 11, 150]);
    let Shown::Parts(d, parts, n) = shown_rows(&next, &shown, 200) else {
        panic!("two runs 139 rows apart are told apart");
    };
    assert_eq!(d, Damage { x: 0, y: 10, width: 4, height: 141 }, "the band round both");
    assert_eq!(&parts[..n], &[(0, 10, 4, 2), (0, 150, 4, 1)]);
}

#[test]
fn rows_changed_close_together_are_one_band() {
    let near = 10 + GAP_ROWS - 1;
    let (next, shown) = frames(200, &[10, near]);
    let band = Damage { x: 0, y: 10, width: 4, height: (near - 9) as u32 };
    assert_eq!(shown_rows(&next, &shown, 200), Shown::Band(band), "fewer than GAP_ROWS apart");
    let (next, shown) = frames(200, &[10, 11 + GAP_ROWS]);
    assert!(matches!(shown_rows(&next, &shown, 200), Shown::Parts(_, _, 2)), "GAP_ROWS apart");
}

#[test]
fn more_runs_than_parts_join_the_closest_and_cover_every_changed_row() {
    // Six runs, the gaps between them 30, 90, 20, 60 and 40 rows.
    let changed = [0, 31, 122, 143, 204, 245];
    let (next, shown) = frames(300, &changed);
    let Shown::Parts(d, parts, n) = shown_rows(&next, &shown, 300) else { panic!("parts") };
    assert_eq!(n, MAX_PARTS);
    assert_eq!(d, Damage { x: 0, y: 0, width: 4, height: 246 });
    assert_eq!(&parts[..n], &[(0, 0, 4, 32), (0, 122, 4, 22), (0, 204, 4, 1), (0, 245, 4, 1)]);
    for r in changed {
        let r = r as u32;
        assert!(parts[..n].iter().any(|&(_, y, _, h)| r >= y && r < y + h), "row {r} covered");
    }
}

#[test]
fn runs_are_read_only_inside_the_span_asked() {
    let (next, shown) = frames(50, &[5, 40]);
    let runs: Vec<_> = runs_in(&next, &shown, 4, (0, 30)).iter().collect();
    assert_eq!(runs, vec![(5, 6)]);
    assert_eq!(runs_in(&next, &shown, 0, (0, 50)).count(), 0);
}

/// Changes in several places at once, as a command's output and the
/// Terminal's status row: over random sessions with held and refused
/// presents the screen is the app after every frame shown, and a frame
/// told apart composes no more than its band.
#[test]
fn frames_told_apart_keep_the_screen_the_app() {
    for seed in 1..=40u64 {
        let mut s = seed.wrapping_mul(0xA24B_AED4_963E_E407) | 1;
        let (mut app, mut new) = (App::new(), New::blank());
        assert!(new.whole(&app));
        for step in 0..120 {
            new.hold = (xorshift(&mut s) % 3) as u32;
            new.refuse_next = xorshift(&mut s).is_multiple_of(19);
            for _ in 0..1 + xorshift(&mut s) % 6 {
                change(&mut s, &mut app);
            }
            let before = new.composed;
            let shown = new.changed(&app);
            new.tick();
            if shown {
                assert!(new.screen == fresh(&app), "seed {seed} step {step}: screen not the app");
                assert!(new.composed - before <= u64::from(W) * u64::from(H));
            }
        }
    }
}

/// The runner bands a frame the app asked for whole in a window whose size,
/// focus, hover and maximized state did not change, and a frame painted
/// again after one not shown.
#[test]
fn the_runner_bands_an_asked_whole_frame_and_one_painted_again() {
    const REPAINT: &str = include_str!("../../../../../app_skeleton/src/runner/repaint.rs");
    assert!(REPAINT.contains("let same = frame == booted.painted;"));
    assert!(REPAINT.contains(
        "true => paint_changed(app, manifest, binding, frame, peers, request_id),\n                false => paint(app, manifest, binding, frame, peers, request_id),"
    ));
    const PRIMED: &str = include_str!("../../../../../app_skeleton/src/runner/ensure_primed.rs");
    assert!(PRIMED.contains(
        "booted.primed = paint_changed(app, manifest, binding, (state, *maximized), peers, request_id);"
    ));
    const FRAME: &str = include_str!("../../../../../app_skeleton/src/runner/paint_frame.rs");
    assert!(FRAME.contains("present_rows(binding.read(i), binding.read(j), rows, (width, height))"));
    assert!(FRAME.contains("None => Shown::Band(Damage { x: 0, y: 0, width, height }),"));
    assert!(FRAME.contains("Shown::Nothing => return binding.swap.cancel(i),"));
    const PRESENT: &str = include_str!("../../../../../app_skeleton/src/runner/present_frame.rs");
    assert!(PRESENT.contains(
        "let on = binding.surface_handle();\n    let pending = binding.swap.present(i, d);"
    ));
}
