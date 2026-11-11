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

//! The player screen at 1x, 1.25x, 1.5x and 2x in a window grown by the same
//! scale: play is drawn in the box its click answers in, the header sits above
//! the picture, and the seek groove answers a little past itself, from the
//! video's start to its end. Its own binary, so setting the scale races no
//! other test.

use apps_proofs::ui::layout::{header_h, layout, seek_permille, seek_reach, top};
use apps_proofs::ui::player::transport::transport;
use apps_proofs::ui::screen::Route;
use apps_proofs::ui::target::{target_at, Target};
use apps_proofs::video_check::{meets, window};
use nonos_toolkit::design::scale::{scale, set_scale};

fn play_answers_where_its_body_is_drawn_and_the_header_sits_above_the_picture(q: u32) {
    let (s, px) = (window(Route::Player), |n| scale().px(n));
    let l = layout(s.dims.0, s.dims.1);
    let play = transport(&l, s.dims.0).play;
    assert_eq!((play.w, play.h), (px(25) * 2, px(38)), "q={q}: the box its click answers in");
    assert!(play.y + play.h <= l.bar.y + l.bar.h, "q={q}: it stays inside the bar");
    assert_eq!(l.video.y, top() + header_h(), "q={q}");
    assert!(!meets(l.back, l.video) && !meets(l.info, l.video), "q={q}");
    let title = (px(400) as i32, (top() + px(20)) as i32);
    assert_eq!(target_at(&s, title.0, title.1), None, "q={q}: the title is no control");
}

fn a_click_just_past_the_groove_still_seeks_from_its_start_to_its_end(q: u32) {
    let s = window(Route::Player);
    let g = layout(s.dims.0, s.dims.1).scrub;
    let band = seek_reach(g);
    assert_eq!(band.h, g.h + scale().px(9) * 2, "q={q}: the reach grows with the scale");
    for (x, y) in [(g.x, band.y), (g.x + g.w - 1, band.y + band.h - 1)] {
        assert_eq!(target_at(&s, x as i32, y as i32), Some(Target::Seek), "q={q}");
    }
    assert_eq!(seek_permille(g, g.x as i32), 0, "q={q}");
    assert_eq!(seek_permille(g, (g.x + g.w - 1) as i32), 1000, "q={q}");
    let half = seek_permille(g, (g.x + (g.w - 1) / 2) as i32);
    assert!(half == 499 || (q != 4 && half == 500), "q={q}: the middle seeks half way");
}

#[test]
fn the_player_answers_where_it_draws_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        play_answers_where_its_body_is_drawn_and_the_header_sits_above_the_picture(q);
        a_click_just_past_the_groove_still_seeks_from_its_start_to_its_end(q);
    }
    set_scale(4);
}
