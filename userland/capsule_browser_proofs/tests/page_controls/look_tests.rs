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
use capsule_browser_proofs::browser::ui::page_part::PagePart;
use nonos_toolkit::widgets::Controls;

use super::fixtures::{bookmarks, downloads, history, tabs};
use super::kit::{at_scale, changed, mid, paint, spots};

/* Each control of `s`, the pointer over its box, changes pixels only in
 * that box, and some, but for the group's name, which is text. */
fn lights_only_in_its_box(s: &Shell, q: u32) {
    let mut c = Controls::acting_on_press();
    let unlit = paint(s, &mut c);
    for (part, r) in spots(&c) {
        let (x, y) = mid(r);
        c.moved(x, y);
        let diff = changed(&unlit, &paint(s, &mut c));
        assert!(part == PagePart::Name || !diff.is_empty(), "{part:?} does not light at {q}");
        let out: Vec<_> = diff.iter().filter(|&&(x, y)| !r.contains(x, y)).take(5).collect();
        assert!(out.is_empty(), "{part:?} lights outside {r:?} at {q}: {out:?}");
        c.left();
    }
}

#[test]
fn each_control_lights_only_in_its_own_box() {
    for q in [4, 8] {
        let _g = at_scale(q);
        for s in [history(), bookmarks(), downloads(), tabs(false), tabs(true)] {
            lights_only_in_its_box(&s, q);
        }
    }
}

#[test]
fn renaming_draws_the_field_where_the_name_acts() {
    for q in [4, 8] {
        let _g = at_scale(q);
        let mut s = tabs(true);
        let mut c = Controls::acting_on_press();
        let plain = paint(&s, &mut c);
        s.rename = Some("Work".into());
        let field = paint(&s, &mut c);
        let name = spots(&c).into_iter().find(|(p, _)| *p == PagePart::Name).unwrap().1;
        let diff = changed(&plain, &field);
        assert!(!diff.is_empty(), "{q}");
        assert!(diff.iter().all(|&(x, y)| name.contains(x, y)), "{name:?} at {q}");
    }
}
