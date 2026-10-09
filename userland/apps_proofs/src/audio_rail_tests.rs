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

//! A click on a row of the audio player's "Up next" rail plays the track that
//! row shows. The rail paints the queue's items in queue order; under shuffle
//! that is not library order, and the click used to select the row number as
//! if it were the library index, playing another track than the one clicked.

use crate::audio_ui::geometry::{shell, Rect};
use crate::audio_ui::shell::rail_geom::{queue_at, queue_row, queue_track_at, queue_visible};

/// The rail of the player's 1440 by 900 window.
fn rail() -> Rect {
    shell(1440, 900).rail
}

/// The middle of the queue's row `i`, as painted.
fn centre(r: Rect, i: usize) -> (i32, i32) {
    let row = queue_row(r, i);
    (row.cx(), row.cy())
}

/// A queue in shuffled order: row 0 shows library track 3, and so on.
const SHUFFLED: [usize; 6] = [3, 0, 5, 1, 4, 2];

#[test]
fn the_window_shows_several_queue_rows() {
    assert!(queue_visible(rail()) >= 3);
}

#[test]
fn a_click_on_a_shuffled_queue_row_picks_the_track_it_shows() {
    let r = rail();
    let shown = queue_visible(r).min(SHUFFLED.len());
    for (row, &track) in SHUFFLED.iter().enumerate().take(shown) {
        let (x, y) = centre(r, row);
        assert_eq!(queue_at(r, x, y), Some(row));
        assert_eq!(queue_track_at(r, &SHUFFLED, x, y), Some(track));
    }
    // The row number is not the track here: the bug this guards.
    let (x, y) = centre(r, 0);
    assert_ne!(queue_track_at(r, &SHUFFLED, x, y), Some(0));
}

#[test]
fn a_click_on_a_queue_in_library_order_picks_that_row() {
    let r = rail();
    let items: Vec<usize> = (0..6).collect();
    for row in 0..queue_visible(r).min(items.len()) {
        let (x, y) = centre(r, row);
        assert_eq!(queue_track_at(r, &items, x, y), Some(row));
    }
}

#[test]
fn a_click_on_a_row_past_the_end_of_the_queue_picks_nothing() {
    let r = rail();
    let items = [4usize];
    let (x, y) = centre(r, 1);
    assert_eq!(queue_at(r, x, y), Some(1));
    assert_eq!(queue_track_at(r, &items, x, y), None);
    assert_eq!(queue_track_at(r, &[], x, y), None);
}

#[test]
fn a_click_above_the_queue_picks_nothing() {
    let r = rail();
    let top = queue_row(r, 0);
    assert_eq!(queue_track_at(r, &SHUFFLED, top.cx(), top.y - 1), None);
}
