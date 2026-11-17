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

//! The video player's Details tabs and Folders page at 1x, 1.25x, 1.5x and 2x
//! in a window grown by the same scale: both tabs answer on the strip that
//! draws them, and Open and the videos are named only when drawn. Its own
//! binary, so setting the scale races no other test.

use apps_proofs::ui::frame::region::body;
use apps_proofs::ui::screen::Route;
use apps_proofs::ui::target::{target_at, Target};
use apps_proofs::ui::targets::{targets, Shown};
use apps_proofs::ui::view::page_geom::strip;
use apps_proofs::video_check::{dims, window, windows};
use nonos_toolkit::design::scale::set_scale;

fn both_details_tabs_answer_on_their_strip_and_nothing_else_does(q: u32) {
    for dims in [dims(960, 600), dims(800, 560)] {
        let s = Shown { dims, ..window(Route::Details) };
        let band = strip(body(dims.0, dims.1));
        let cy = (band.y + band.h / 2) as i32;
        let at = |x: u32| target_at(&s, x as i32, cy);
        let named: Vec<_> = (band.x..band.x + band.w).map(at).collect();
        for i in 0..2 {
            assert!(named.contains(&Some(Target::Tab(i))), "q={q}: tab {i} at {dims:?}");
        }
        assert!(named.iter().all(|t| matches!(t, None | Some(Target::Tab(_)))), "q={q}");
    }
}

fn the_open_button_and_the_videos_are_named_only_when_drawn(q: u32) {
    for s in windows().into_iter().filter(|s| s.route == Route::Files) {
        let ts = targets(&s);
        let open = ts.iter().any(|(t, _)| *t == Target::Open);
        let videos = ts.iter().any(|(t, _)| matches!(t, Target::Tile(_) | Target::Row(_)));
        assert_eq!((open, videos), (s.len > 0, s.len > 0), "q={q}: {s:?}");
    }
}

#[test]
fn the_tabs_and_the_folders_page_answer_where_drawn_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        both_details_tabs_answer_on_their_strip_and_nothing_else_does(q);
        the_open_button_and_the_videos_are_named_only_when_drawn(q);
    }
    set_scale(4);
}
