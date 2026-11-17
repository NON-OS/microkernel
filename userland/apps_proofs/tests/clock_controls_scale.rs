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

//! The clock keeps logical metrics and draws them at the display scale: at
//! 1x, 1.25x, 1.5x and 2x the tab bar and every button are their 1x rects
//! through `scale().px` and sit in a window grown by the same scale, what a
//! click or a hover names is the control drawn there, the buttons of one tab
//! never overlap, and each tab of the strip names itself. Its own binary, so
//! setting the scale races no other test.

use apps_proofs::clock::controls::{keys, target_at, Target};
use apps_proofs::clock::manifest::{HEIGHT, WIDTH};
use apps_proofs::clock::tabs::{all, bar, strip, Tab};
use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::widgets::rect::contains;
use nonos_toolkit::widgets::Rect;

fn grown(r: Rect) -> Rect {
    let s = scale();
    Rect { x: s.px(r.x), y: s.px(r.y), w: s.px(r.w), h: s.px(r.h) }
}

fn every_size_is_its_1x_size_and_fits(at_1x: &[(Vec<Rect>, Rect)], w: u32, h: u32) {
    for (tab, (ks, b)) in all().into_iter().zip(at_1x) {
        let want: Vec<Rect> = ks.iter().map(|&k| grown(k)).collect();
        assert_eq!(keys(tab), want, "a button is not its 1x rect at the scale");
        assert_eq!(bar(w), Rect { w, ..grown(*b) }, "the bar is not its 1x rect at the scale");
        for k in keys(tab) {
            assert!(k.x + k.w <= w && k.y + k.h <= h, "{k:?} leaves the {w}x{h} window");
            assert!(k.y >= b.y + b.h, "{k:?} overlaps the tab bar");
        }
    }
}

fn every_button_is_named_by_the_point_at_its_centre(w: u32) {
    for tab in all() {
        for (i, k) in keys(tab).iter().enumerate() {
            let (x, y) = ((k.x + k.w / 2) as i32, (k.y + k.h / 2) as i32);
            assert_eq!(target_at(tab, w, x, y), Some(Target::Key(i)));
        }
    }
}

fn the_buttons_of_one_tab_never_overlap() {
    for tab in all() {
        let ks = keys(tab);
        for (i, a) in ks.iter().enumerate() {
            for b in &ks[i + 1..] {
                let corners = [(b.x, b.y), (b.x + b.w - 1, b.y), (b.x, b.y + b.h - 1)];
                assert!(!corners.iter().any(|&(x, y)| contains(*a, x as i32, y as i32)));
            }
        }
    }
}

fn each_tab_of_the_strip_names_itself_and_nothing_names_a_tab_off_the_bar(w: u32) {
    let b = bar(w);
    for tab in all() {
        let names: Vec<_> =
            (0..w as i32).filter_map(|x| target_at(tab, w, x, (b.y + b.h / 2) as i32)).collect();
        for i in 0..all().len() {
            assert!(names.contains(&Target::Tab(i)), "{i} has no point on the bar");
        }
        assert_eq!(strip(tab).hit(b, 10, b.y as i32 - 1), None);
    }
}

fn the_clock_tab_has_no_buttons_and_a_click_off_everything_names_nothing(w: u32, h: u32) {
    assert!(keys(Tab::Clock).is_empty());
    assert_eq!(target_at(Tab::Timer, w, 5, h as i32), None);
}

#[test]
fn the_controls_scale_and_still_name_what_is_drawn_at_every_scale() {
    set_scale(4);
    let at_1x: Vec<(Vec<Rect>, Rect)> = all().into_iter().map(|t| (keys(t), bar(WIDTH))).collect();
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let (w, h) = (scale().px(WIDTH), scale().px(HEIGHT));
        every_size_is_its_1x_size_and_fits(&at_1x, w, h);
        every_button_is_named_by_the_point_at_its_centre(w);
        the_buttons_of_one_tab_never_overlap();
        each_tab_of_the_strip_names_itself_and_nothing_names_a_tab_off_the_bar(w);
        the_clock_tab_has_no_buttons_and_a_click_off_everything_names_nothing(w, h);
    }
    set_scale(4);
}
