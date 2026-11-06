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
//! The audio player's lists and grooves: the Library names each row it draws
//! under the scroll and each tab of its strip, and the pointer sweeping the
//! seek bar, past its ends too, stays on it and repaints once.

use nonos_app_skeleton::InputKind::PointerAbs;

use crate::audio_check::{feed, points, window};
use crate::audio_ui::geometry::{page, shell, Rect};
use crate::audio_ui::hover::Hover;
use crate::audio_ui::screen::lib_geom::{list_rect, tabs_rect, visible, ROW_H};
use crate::audio_ui::shell::bar::bar;
use crate::audio_ui::shown::{target_at, Shown};
use crate::audio_ui::target::{Key, Target};
use crate::audio_ui::view::View;

#[test]
fn the_library_names_each_row_it_draws_under_the_scroll() {
    let s = Shown { scroll: 3, ..window(View::Library) };
    let l = list_rect(page(&shell(1440, 900)));
    for slot in 0..visible(page(&shell(1440, 900))) {
        let row = Rect::new(l.x, l.y + slot as i32 * ROW_H, l.w, ROW_H);
        assert!(points(row)
            .iter()
            .all(|&(x, y)| target_at(&s, x, y) == Some(Target::Row(3 + slot))));
    }
}

#[test]
fn each_tab_of_the_strip_names_itself_and_nothing_above_it_does() {
    let s = window(View::Library);
    let strip = tabs_rect(page(&shell(1440, 900)));
    let named: Vec<_> =
        (strip.x..strip.right()).filter_map(|x| target_at(&s, x, strip.cy())).collect();
    assert!((0..4).all(|i| named.contains(&Target::Tab(i))));
    assert!((strip.x..strip.right()).all(|x| target_at(&s, x, strip.y - 1).is_none()));
}

#[test]
fn sweeping_the_seek_bar_and_ten_past_each_end_repaints_once() {
    let (s, mut h) = (window(View::Home), Hover::new());
    let scrub = bar(shell(1440, 900).transport).scrub;
    let sweep = (scrub.x - 10..scrub.right() + 10).map(|x| (x, scrub.cy()));
    let lit = sweep.filter(|&p| feed(&mut h, &s, PointerAbs, p)).count();
    assert_eq!((lit, h.over), (1, Some(Target::Bar(Key::Seek))));
    assert!(feed(&mut h, &s, PointerAbs, (scrub.right() + 10, scrub.cy())));
}
