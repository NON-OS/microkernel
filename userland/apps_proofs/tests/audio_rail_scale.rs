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
//! row shows, at every scale. The rail paints the queue's items in queue
//! order; under shuffle that is not library order, and the click used to pick
//! the row number as if it were the library index.

use apps_proofs::audio_check::dims;
use apps_proofs::audio_ui::geometry::{shell, Rect};
use apps_proofs::audio_ui::shell::rail_geom::{queue_at, queue_row, queue_track_at, queue_visible};
use nonos_toolkit::design::scale::set_scale;

/// A queue in shuffled order: row 0 shows library track 3, and so on.
const SHUFFLED: [usize; 6] = [3, 0, 5, 1, 4, 2];

fn centre(r: Rect, i: usize) -> (i32, i32) {
    let row = queue_row(r, i);
    (row.cx(), row.cy())
}

fn rows_pick_what_they_show(r: Rect, q: u32) {
    assert!(queue_visible(r) >= 3, "q={q}: the rail shows too few rows");
    let shown = queue_visible(r).min(SHUFFLED.len());
    for (row, &track) in SHUFFLED.iter().enumerate().take(shown) {
        let (x, y) = centre(r, row);
        assert_eq!(queue_at(r, x, y), Some(row), "q={q}");
        assert_eq!(queue_track_at(r, &SHUFFLED, x, y), Some(track), "q={q}");
    }
    let (x, y) = centre(r, 0);
    assert_ne!(queue_track_at(r, &SHUFFLED, x, y), Some(0), "q={q}: the row number");
    let items: Vec<usize> = (0..6).collect();
    for row in 0..queue_visible(r).min(items.len()) {
        let (x, y) = centre(r, row);
        assert_eq!(queue_track_at(r, &items, x, y), Some(row), "q={q}");
    }
}

fn past_the_queue_picks_nothing(r: Rect, q: u32) {
    let (x, y) = centre(r, 1);
    assert_eq!(queue_at(r, x, y), Some(1), "q={q}");
    assert_eq!(queue_track_at(r, &[4], x, y), None, "q={q}: past the end");
    assert_eq!(queue_track_at(r, &[], x, y), None, "q={q}: an empty queue");
    let top = queue_row(r, 0);
    assert_eq!(queue_track_at(r, &SHUFFLED, top.cx(), top.y - 1), None, "q={q}: above");
}

#[test]
fn a_click_on_the_rail_picks_the_track_its_row_shows_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let (w, h) = dims(1440, 900);
        let r = shell(w, h).rail;
        rows_pick_what_they_show(r, q);
        past_the_queue_picks_nothing(r, q);
    }
    set_scale(4);
}
