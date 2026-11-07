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

//! Setup committed its whole screen on every key: a typed letter of the
//! passphrase, a highlight moved one row. On a development guest under TCG
//! that was a whole 1920x1080 compose for each of about a hundred keys of a
//! setup walk. Each frame is drawn off screen now, the rows that differ are
//! copied, and only their bands are committed (row_bands.rs, show.rs).

use crate::row_bands::{Bands, JOIN_GAP, MAX_BANDS};

fn bands_of(changed: &[usize]) -> Bands {
    let mut b = Bands::new();
    changed.iter().for_each(|&y| b.add(y));
    b
}

fn covers(b: &Bands, y: usize) -> bool {
    b.as_slice().iter().any(|band| y >= band.y && y < band.y + band.rows)
}

/// A small xorshift, so the runs are the same each time.
fn next(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

#[test]
fn every_changed_row_is_in_a_band_and_the_bands_are_few_and_in_order() {
    let mut seed = 0x9E37_79B9_7F4A_7C15u64;
    for _ in 0..2000 {
        let rows = 1 + (next(&mut seed) % 1200) as usize;
        let mut changed: Vec<usize> = (0..rows).filter(|_| next(&mut seed) % 23 == 0).collect();
        changed.dedup();
        let b = bands_of(&changed);
        assert!(b.as_slice().len() <= MAX_BANDS);
        for &y in &changed {
            assert!(covers(&b, y), "row {y} changed and was not committed");
        }
        for w in b.as_slice().windows(2) {
            assert!(w[0].y + w[0].rows <= w[1].y, "bands overlap or are out of order");
        }
        for band in b.as_slice() {
            assert!(band.rows > 0);
            assert!(changed.contains(&band.y), "a band starts on a row that did not change");
            assert!(
                changed.contains(&(band.y + band.rows - 1)),
                "a band ends past its last change"
            );
        }
    }
}

#[test]
fn a_frame_the_same_as_the_screen_commits_nothing() {
    assert!(bands_of(&[]).as_slice().is_empty());
}

#[test]
fn a_typed_letter_commits_its_line_not_the_screen() {
    // The passphrase field's text line, 28 rows of a 1080-row screen.
    let line: Vec<usize> = (610..638).collect();
    let b = bands_of(&line);
    assert_eq!(b.as_slice().len(), 1);
    assert_eq!((b.as_slice()[0].y, b.as_slice()[0].rows), (610, 28));
}

#[test]
fn rows_a_little_apart_join_and_rows_far_apart_stay_apart() {
    let near = bands_of(&[100, 100 + JOIN_GAP + 1]);
    assert_eq!(near.as_slice().len(), 1, "a gap of JOIN_GAP rows joins");
    let far = bands_of(&[100, 100 + JOIN_GAP + 2]);
    assert_eq!(far.as_slice().len(), 2);
    // A highlight moved from one row of a list to another: two bands.
    let moved: Vec<usize> = (300..330).chain(500..530).collect();
    let b = bands_of(&moved);
    let rows: usize = b.as_slice().iter().map(|band| band.rows).sum();
    assert_eq!((b.as_slice().len(), rows), (2, 60));
}

#[test]
fn past_the_last_band_the_last_one_reaches_down() {
    let spread: Vec<usize> = (0..10).map(|i| i * 100).collect();
    let b = bands_of(&spread);
    assert_eq!(b.as_slice().len(), MAX_BANDS);
    let last = b.as_slice()[MAX_BANDS - 1];
    assert_eq!((last.y, last.y + last.rows), (300, 901));
}

#[test]
fn a_frame_drawn_on_the_surface_is_one_band_of_every_row() {
    let b = Bands::whole(1080);
    assert_eq!(b.as_slice().len(), 1);
    assert_eq!((b.as_slice()[0].y, b.as_slice()[0].rows), (0, 1080));
    assert!(Bands::whole(0).as_slice().is_empty());
}

#[test]
fn setup_draws_off_screen_and_commits_only_what_changed() {
    const RUNNER: &str = include_str!("../../capsule_setup_wizard/src/server/runner.rs");
    const JOIN: &str = include_str!("../../capsule_setup_wizard/src/network/join.rs");
    const SHOW: &str = include_str!("../../capsule_setup_wizard/src/render/show.rs");
    const FRAME: &str = include_str!("../../capsule_setup_wizard/src/render/frame.rs");
    const RUN: &str = include_str!("../../capsule_setup_wizard/src/setup/run.rs");
    for src in [RUNNER, JOIN] {
        assert!(src.contains("screens::draw(ctx);\n"));
        assert!(src.contains("crate::render::show::show(ctx, "));
        assert!(!src.contains("damage_commit("), "no whole-screen commit after a draw");
    }
    assert!(
        SHOW.contains("if s != b {\n            s.copy_from_slice(b);\n            bands.add(y);")
    );
    assert!(SHOW.contains(
        "compositor::damage_rect(ctx.compositor_port, request_id, 0, y, ctx.width, rows)"
    ));
    assert!(SHOW.contains("if ctx.back == 0 {\n        return Bands::whole(rows);"));
    assert!(FRAME.contains("let at = if ctx.back != 0 { ctx.back } else { ctx.base };"));
    assert!(RUN.contains("ctx.back = mapped(back).unwrap_or(0);"));
    assert!(RUN.contains("let Some(backing_va) = mapped(base) else {"));
    assert!(RUN.contains("if !shape_ok(width, height, stride) {"));
}
