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

//! The wallet's frame in a small window and at full screen: the column takes
//! the window less a margin up to a readable width, the type grows with it,
//! a wide window is two columns, and content never meets the edges.

use crate::layout::{
    content, content_bottom, halves, layout, MARGIN, MAX_COLUMN, PHOTO_W, TWO_UP_FROM,
};
use crate::status_words::unread_balances;

/// The frame's own spacing (`etna::tokens`): the content's inset in the
/// column, the room above the foot, the gap between two columns.
const SIDE: u32 = 24;
const ROOM: u32 = 28;
const GAP: u32 = 26;

#[test]
fn a_small_window_is_one_column_inside_its_margins() {
    let l = layout(800);
    assert_eq!(l.column, 680, "the photographs' width and half of the rest");
    assert_eq!((l.x, 800 - l.x - l.column), (60, 60), "the margins are equal");
    assert!(!l.two_up);
    assert_eq!(l.scale_pct, 100);
    let (cx, cw) = content(&l, SIDE);
    assert_eq!((cx, cw), (84, 632));
    assert!(cx >= l.x + SIDE && cx + cw <= l.x + l.column - SIDE);
    assert_eq!(content_bottom(600, 44, ROOM), 528, "room above a status-only foot");
}

#[test]
fn full_screen_is_two_columns_at_a_readable_width_with_larger_type() {
    let l = layout(1920);
    assert_eq!(l.column, MAX_COLUMN);
    assert_eq!(l.x, (1920 - MAX_COLUMN) / 2, "centred");
    assert!(l.two_up);
    assert_eq!(l.scale_pct, 125);
    let (cx, cw) = content(&l, SIDE);
    let ((lx, lw), (rx, rw)) = halves(cx, cw, 2 * GAP);
    assert_eq!(lx, cx);
    assert_eq!(rx + rw, cx + cw, "the right column ends where the content does");
    assert_eq!(rx - (lx + lw), 2 * GAP, "the gap between them is kept");
    assert!(lw.abs_diff(rw) <= 1, "the two columns are the same width");
    assert!(lw >= 400, "each column holds a tile of balances");
    assert_eq!(content_bottom(1080, 44, ROOM), 1008);
}

#[test]
fn every_width_lays_a_column_inside_the_window() {
    for w in (320..=3840).step_by(16) {
        let l = layout(w);
        assert!(l.x + l.column <= w, "{w}: the column fits");
        assert!(
            l.x <= MARGIN || l.column == MAX_COLUMN,
            "{w}: margins only past the widest column"
        );
        assert!(l.column <= MAX_COLUMN, "{w}: never past the readable width");
        assert!(l.column >= w.min(PHOTO_W), "{w}: never narrower than the photographs");
        assert_eq!(l.two_up, l.column >= TWO_UP_FROM);
        assert!((100..=125).contains(&l.scale_pct));
        let (cx, cw) = content(&l, SIDE);
        assert!(cx + cw <= w && cw > 0, "{w}: the content fits");
    }
}

#[test]
fn the_column_and_its_type_never_shrink_as_the_window_grows() {
    let mut last = layout(320);
    for w in 321..=3840 {
        let l = layout(w);
        assert!(l.column >= last.column, "{w}");
        assert!(l.scale_pct >= last.scale_pct, "{w}");
        last = l;
    }
}

#[test]
fn unread_balances_are_one_honest_line() {
    assert_eq!(unread_balances(false, false), "offline, balance not checked");
    assert_eq!(unread_balances(false, true), "not read yet");
    assert_eq!(unread_balances(true, false), "reading the network");
}

#[test]
fn a_kept_note_reads_as_a_sentence_on_the_screen() {
    use crate::keep_plan::{kept_status, Kept};
    use crate::status_words::capital;
    let said = core::str::from_utf8(kept_status(Kept::Live, false).unwrap()).unwrap();
    let shown = capital(said);
    assert!(shown.starts_with("This is a live session"));
    assert!(shown.ends_with("write down the phrase."));
    assert_eq!(capital(""), ".");
}
