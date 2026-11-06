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
//! The audio player's controls: every one in the table is named at its
//! middle and corners, no two overlap, and the Home banner's buttons answer
//! where the banner draws them, in every view.

use crate::audio_check::{meets, points, window, windows};
use crate::audio_ui::geometry::{page, shell, Rect};
use crate::audio_ui::screen::home_geom::{action_rect, HERO_H};
use crate::audio_ui::shown::{target_at, Shown};
use crate::audio_ui::target::Target;
use crate::audio_ui::targets::targets;
use crate::audio_ui::view::View;

#[test]
fn every_control_is_named_at_its_middle_and_its_corners() {
    for s in windows() {
        for (t, r) in targets(&s).into_iter().filter(|(_, r)| r.h > 0 && r.w > 0) {
            for (x, y) in points(r) {
                assert_eq!(target_at(&s, x, y), Some(t), "{t:?} at {x},{y} in {:?}", s.view);
            }
        }
    }
}

#[test]
fn no_two_controls_overlap_but_the_clear_mark_inside_the_search_field() {
    for s in windows() {
        let ts = targets(&s);
        for (i, &(a, ra)) in ts.iter().enumerate() {
            let on_field = |b| (a, b) == (Target::Clear, Target::Field);
            let clear = ts[i + 1..].iter().all(|&(b, rb)| on_field(b) || !meets(ra, rb));
            assert!(clear, "{a:?} meets another in {:?}", s.view);
        }
    }
}

#[test]
fn the_banner_buttons_answer_where_the_banner_draws_them() {
    let p = page(&shell(1440, 900));
    let banner = Rect::new(p.x, p.y, p.w, HERO_H);
    for s in [window(View::Home), Shown { tracks: 0, ..window(View::Home) }] {
        for i in 0..2 {
            let (drawn, page_foot) = (action_rect(banner, i), action_rect(p, i));
            assert_eq!(target_at(&s, drawn.cx(), drawn.cy()), Some(Target::Hero(i)));
            assert_ne!(target_at(&s, page_foot.cx(), page_foot.cy()), Some(Target::Hero(i)));
        }
    }
}

#[test]
fn the_bar_offers_eight_controls_and_the_rail_eight_more_when_it_shows() {
    let keys = |s: Shown| {
        targets(&s).iter().filter(|(t, _)| matches!(t, Target::Bar(_) | Target::Rail(_))).count()
    };
    assert_eq!(keys(window(View::NowPlaying)), 16);
    assert_eq!(keys(Shown { dims: (1000, 700), ..window(View::NowPlaying) }), 8);
}
