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

//! The layout at every width, and the status lines the frame shows.

use super::layout_frame::SIDE;
use crate::layout::{content, layout, MARGIN, MAX_COLUMN, PHOTO_W, TWO_UP_FROM};
use crate::status_words::unread_balances;

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
