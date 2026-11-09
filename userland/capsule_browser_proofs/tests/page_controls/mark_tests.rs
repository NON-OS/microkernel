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

use capsule_browser_proofs::browser::shell::Shell;
use capsule_browser_proofs::browser::ui::page_part::{PagePart, Pick};
use capsule_browser_proofs::browser::ui::rect::Rect;
use nonos_toolkit::widgets::Controls;

use super::fixtures::{downloads, history, tabs};
use super::found::id;
use super::kit::{at_scale, changed, geom, paint, spots};

/* The boxes of `parts` as `s` lays them out. */
fn boxes(s: &Shell, parts: &[PagePart]) -> Vec<Rect> {
    let mut c = Controls::acting_on_press();
    paint(s, &mut c);
    spots(&c).into_iter().filter(|(p, _)| parts.contains(p)).map(|(_, r)| r).collect()
}

/* Between `a` and `b`, what moves in the strip `rows` lies only in `on`,
 * and some of it in each of them. */
fn marks_only(a: &Shell, b: &Shell, rows: (i32, i32), on: &[PagePart], q: u32) {
    let mut c = Controls::acting_on_press();
    let diff = changed(&paint(a, &mut c), &paint(b, &mut c));
    let near: Vec<_> = diff.into_iter().filter(|&(_, y)| y >= rows.0 && y < rows.1).collect();
    let (ra, rb) = (boxes(a, on), boxes(b, on));
    assert!(!near.is_empty(), "{on:?} at {q}");
    let inside = |&(x, y): &(i32, i32)| ra.iter().chain(&rb).any(|r| r.contains(x, y));
    assert!(near.iter().all(inside), "{on:?} at {q}");
    for r in &ra {
        assert!(near.iter().any(|&(x, y)| r.contains(x, y)), "{r:?} of {on:?} at {q}");
    }
}

#[test]
fn the_filter_or_tab_in_force_is_the_only_one_marked() {
    for q in [4, 8] {
        let _g = at_scale(q);
        let (h, d, t) = (history(), downloads(), tabs(false));
        let pills = |s: &Shell| (geom(s).pills_y, geom(s).pills_y + geom(s).pill_h);
        for i in 0..3u8 {
            let two = [PagePart::Pill(Pick::Filter(i)), PagePart::Pill(Pick::Filter(i + 1))];
            let from = Shell { hist_filter: i, ..history() };
            marks_only(&from, &Shell { hist_filter: i + 1, ..history() }, pills(&h), &two, q);
        }
        let two = [PagePart::Pill(Pick::Filter(0)), PagePart::Pill(Pick::Filter(2))];
        marks_only(&d, &Shell { dl_filter: 2, ..downloads() }, pills(&d), &two, q);
        let mut moved = tabs(false);
        moved.cards[1].active = false;
        moved.cards[3].active = true;
        let two = [PagePart::Tab(1, id(&t, 1)), PagePart::Tab(3, id(&t, 3))];
        marks_only(&t, &moved, (0, super::kit::H), &two, q);
    }
}
