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

//! The player screen: play is drawn in the box its click answers in, the
//! header sits above the picture, and the seek groove answers a little past
//! itself, from the video's start to its end, as its click always did.

use crate::ui::layout::{layout, seek_permille, seek_reach, HEADER_H, TOP};
use crate::ui::player::transport::transport;
use crate::ui::screen::Route;
use crate::ui::target::{target_at, Target};
use crate::video_check::{meets, window};

#[test]
fn play_answers_where_its_body_is_drawn_and_the_header_sits_above_the_picture() {
    let s = window(Route::Player);
    let l = layout(s.dims.0, s.dims.1);
    let play = transport(&l, s.dims.0).play;
    assert_eq!(play.h, 38, "the body is the box the click has always answered in");
    assert!(play.y + play.h <= l.bar.y + l.bar.h, "it stays inside the bar");
    assert_eq!(l.video.y, TOP + HEADER_H);
    assert!(!meets(l.back, l.video) && !meets(l.info, l.video));
    assert_eq!(target_at(&s, 400, (TOP + 20) as i32), None, "the title is no control");
}

#[test]
fn a_click_just_past_the_groove_still_seeks_from_its_start_to_its_end() {
    let s = window(Route::Player);
    let g = layout(s.dims.0, s.dims.1).scrub;
    let band = seek_reach(g);
    for (x, y) in [(g.x, band.y), (g.x + g.w - 1, band.y + band.h - 1)] {
        assert_eq!(target_at(&s, x as i32, y as i32), Some(Target::Seek));
    }
    assert_eq!(seek_permille(g, g.x as i32), 0);
    assert_eq!(seek_permille(g, (g.x + g.w - 1) as i32), 1000);
    assert_eq!(seek_permille(g, (g.x + (g.w - 1) / 2) as i32), 499);
}
