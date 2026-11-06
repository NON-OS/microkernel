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

//! The video player's controls: each is named where it is drawn, at its
//! middle and corners, no two overlap, the Details tabs answer on the strip
//! that draws them, and Open and the videos are named only when drawn.

use crate::ui::frame::region::body;
use crate::ui::screen::Route;
use crate::ui::target::{target_at, Target};
use crate::ui::targets::{targets, Shown};
use crate::ui::view::page_geom::strip;
use crate::video_check::{meets, points, window, windows};

#[test]
fn every_control_is_named_at_its_middle_and_its_corners() {
    for s in windows() {
        for (t, r) in targets(&s).into_iter().filter(|(_, r)| r.w > 0 && r.h > 0) {
            for (x, y) in points(r) {
                assert_eq!(target_at(&s, x, y), Some(t), "{t:?} at {x},{y} in {s:?}");
            }
        }
    }
}

#[test]
fn no_two_controls_overlap() {
    for s in windows() {
        let ts = targets(&s);
        for (i, &(a, ra)) in ts.iter().enumerate() {
            assert!(ts[i + 1..].iter().all(|&(_, rb)| !meets(ra, rb)), "{a:?} in {s:?}");
        }
    }
}

#[test]
fn both_details_tabs_answer_on_their_strip_and_nothing_else_does() {
    for dims in [(960, 600), (800, 560)] {
        let s = Shown { dims, ..window(Route::Details) };
        let band = strip(body(dims.0, dims.1));
        let cy = (band.y + band.h / 2) as i32;
        let at = |x: u32| target_at(&s, x as i32, cy);
        let named: Vec<_> = (band.x..band.x + band.w).map(at).collect();
        for i in 0..2 {
            assert!(named.contains(&Some(Target::Tab(i))), "tab {i} at {dims:?}");
        }
        assert!(named.iter().all(|t| matches!(t, None | Some(Target::Tab(_)))));
    }
}

#[test]
fn the_open_button_and_the_videos_are_named_only_when_drawn() {
    for s in windows().into_iter().filter(|s| s.route == Route::Files) {
        let ts = targets(&s);
        let open = ts.iter().any(|(t, _)| *t == Target::Open);
        let videos = ts.iter().any(|(t, _)| matches!(t, Target::Tile(_) | Target::Row(_)));
        assert_eq!((open, videos), (s.len > 0, s.len > 0), "{s:?}");
    }
}
