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

//! The audio player's lists and grooves at every scale: the Library names each
//! row it draws under the scroll and each tab of its strip, and the pointer
//! sweeping the seek bar and its reach past each end stays on it and repaints
//! once.

use apps_proofs::audio_check::{dims, feed, points, window};
use apps_proofs::audio_ui::geometry::{page, shell, Rect, Shell};
use apps_proofs::audio_ui::hover::Hover;
use apps_proofs::audio_ui::screen::lib_geom::{list_rect, row_h, tabs_rect, visible};
use apps_proofs::audio_ui::shell::bar::bar;
use apps_proofs::audio_ui::shown::{target_at, Shown};
use apps_proofs::audio_ui::target::{Key, Target};
use apps_proofs::audio_ui::view::View;
use nonos_app_skeleton::InputKind::PointerAbs;
use nonos_toolkit::design::scale::{scale, set_scale};

fn rows_named_under_the_scroll(sh: &Shell, q: u32) {
    let s = Shown { scroll: 3, ..window(View::Library) };
    let (l, step) = (list_rect(page(sh)), row_h());
    for slot in 0..visible(page(sh)) {
        let row = Rect::new(l.x, l.y + slot as i32 * step, l.w, step);
        let named = |(x, y): (i32, i32)| target_at(&s, x, y) == Some(Target::Row(3 + slot));
        assert!(points(row).into_iter().all(named), "q={q}: row {slot}");
    }
}

fn tabs_name_themselves(sh: &Shell, q: u32) {
    let s = window(View::Library);
    let strip = tabs_rect(page(sh));
    let named: Vec<_> =
        (strip.x..strip.right()).filter_map(|x| target_at(&s, x, strip.cy())).collect();
    assert!((0..4).all(|i| named.contains(&Target::Tab(i))), "q={q}: a tab is not named");
    let above = (strip.x..strip.right()).all(|x| target_at(&s, x, strip.y - 1).is_none());
    assert!(above, "q={q}: a point above the strip names a control");
}

fn seek_sweep_repaints_once(sh: &Shell, q: u32) {
    let (s, mut h) = (window(View::Home), Hover::new());
    let (scrub, reach) = (bar(sh.transport).scrub, scale().px(10) as i32);
    let sweep = (scrub.x - reach..scrub.right() + reach).map(|x| (x, scrub.cy()));
    let lit = sweep.filter(|&p| feed(&mut h, &s, PointerAbs, p)).count();
    assert_eq!((lit, h.over), (1, Some(Target::Bar(Key::Seek))), "q={q}");
    assert!(feed(&mut h, &s, PointerAbs, (scrub.right() + reach, scrub.cy())), "q={q}");
}

#[test]
fn the_lists_and_the_seek_bar_answer_where_they_are_drawn_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let (w, h) = dims(1440, 900);
        let sh = shell(w, h);
        rows_named_under_the_scroll(&sh, q);
        tabs_name_themselves(&sh, q);
        seek_sweep_repaints_once(&sh, q);
    }
    set_scale(4);
}
