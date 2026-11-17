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

//! The audio player's controls at every scale: each is named at its middle and
//! corners, none overlap, and the banner's buttons answer where they are drawn.

use apps_proofs::audio_check::{dims, meets, points, window, windows};
use apps_proofs::audio_ui::geometry::{page, shell, Rect};
use apps_proofs::audio_ui::screen::home_geom::{action_rect, hero_h};
use apps_proofs::audio_ui::shown::{target_at, Shown};
use apps_proofs::audio_ui::target::Target::{self, Bar, Hero, Rail};
use apps_proofs::audio_ui::targets::targets;
use apps_proofs::audio_ui::view::View;
use nonos_toolkit::design::scale::set_scale;

fn named_at_middle_and_corners(q: u32) {
    for s in windows() {
        for (t, r) in targets(&s).into_iter().filter(|(_, r)| r.h > 0 && r.w > 0) {
            for (x, y) in points(r) {
                assert_eq!(target_at(&s, x, y), Some(t), "q={q}: {t:?} at {x},{y} in {:?}", s.view);
            }
        }
    }
}

fn no_overlap_but_the_clear_mark(q: u32) {
    for s in windows() {
        let ts = targets(&s);
        for (i, &(a, ra)) in ts.iter().enumerate() {
            let on_field = |b| (a, b) == (Target::Clear, Target::Field);
            let clear = ts[i + 1..].iter().all(|&(b, rb)| on_field(b) || !meets(ra, rb));
            assert!(clear, "q={q}: {a:?} meets another in {:?}", s.view);
        }
    }
}

fn banner_buttons_answer_where_drawn(q: u32) {
    let p = page(&shell(dims(1440, 900).0, dims(1440, 900).1));
    let banner = Rect::new(p.x, p.y, p.w, hero_h());
    for s in [window(View::Home), Shown { tracks: 0, ..window(View::Home) }] {
        for i in 0..2 {
            let (drawn, page_foot) = (action_rect(banner, i), action_rect(p, i));
            assert_eq!(target_at(&s, drawn.cx(), drawn.cy()), Some(Hero(i)), "q={q}");
            assert_ne!(target_at(&s, page_foot.cx(), page_foot.cy()), Some(Hero(i)), "q={q}");
        }
    }
}

#[test]
fn every_control_answers_where_it_is_drawn_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        named_at_middle_and_corners(q);
        no_overlap_but_the_clear_mark(q);
        banner_buttons_answer_where_drawn(q);
        let n =
            |s: Shown| targets(&s).iter().filter(|(t, _)| matches!(t, Bar(_) | Rail(_))).count();
        assert_eq!(n(window(View::NowPlaying)), 16, "q={q}");
        assert_eq!(n(Shown { dims: dims(1000, 700), ..window(View::NowPlaying) }), 8, "q={q}");
    }
    set_scale(4);
}
