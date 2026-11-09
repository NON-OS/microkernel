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

//! Toast: asking for `TOAST_W` and `TOAST_LINES` gives what an unasked
//! toast gives, size, pixels and hits; and a wider toast holding more lines
//! stays in every box, hits exactly the action it draws, at every scale.

use nonos_toolkit::design::{scale::set_scale, size::{TOAST_LINES, TOAST_W}};
use nonos_toolkit::widgets::Toast;

use super::bounds::{assert_bounded, assert_bounded_in};
use super::{boxes::{boxed, boxes}, canvas::Canvas};
use super::menu_tag_hit::around;
use super::serial::{serial, SCALES};
use super::toast::{ACTIONS, OVER, TEXTS};
use super::toast_hit::wrong;
use super::toast_wide_lines::NOTICE;

#[test]
fn toast_asked_for_the_defaults_is_unchanged() {
    let _g = serial();
    for q in SCALES {
        set_scale(q);
        for (text, action) in TEXTS.into_iter().flat_map(|t| ACTIONS.map(|a| (t, a))) {
            let t = Toast::new(text).action(action);
            let same = t.max_width(TOAST_W).text_lines(TOAST_LINES);
            assert_eq!(t.size(), same.size(), "{text:?} {action:?} at {q}/4");
            for at in boxes(t.size()) {
                let paint = |t: Toast| Canvas::new(at, 0).painted(|fb| t.draw(fb, at)).px;
                let moved = |&(x, y): &(i32, i32)| t.hit(at, x, y) != same.hit(at, x, y);
                let hit = around(at).step_by(5).find(moved);
                let what = format!("{text:?} {action:?} at {q}/4 in {at:?}: {hit:?}");
                assert!(paint(t) == paint(same) && hit.is_none(), "{what}");
            }
        }
    }
    set_scale(SCALES[0]);
}

/// Wide toasts asking for one line, three and more than any text needs.
fn wide() -> Vec<(String, Toast<'static>)> {
    let each = [NOTICE, OVER].into_iter().flat_map(|t| ["", "Undo"].map(|a| (t, a)));
    let toast = |(t, a): (&'static str, &'static str), n| Toast::new(t).action(a).text_lines(n);
    each.flat_map(|ta| [1, 3, 9].map(|n| (format!("{ta:?} {n}"), toast(ta, n).max_width(720))))
        .collect()
}

#[test]
fn toast_wide_and_tall_stays_in_its_box_and_hits_its_action() {
    let _g = serial();
    assert_bounded(&wide());
    assert_bounded_in(&wide(), |(w, h)| [boxed(w, h - 1), boxed(w, h / 2), boxed(w / 3, 2)]);
    for q in SCALES {
        set_scale(q);
        for (name, t) in wide() {
            for at in boxes(t.size()) {
                assert_eq!(wrong(t, at), None, "{name} at {q}/4 in {at:?}");
            }
        }
    }
    set_scale(SCALES[0]);
}
