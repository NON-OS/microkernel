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
//! Field: while it has the keyboard, the caret at every index of a text too
//! long for it is drawn inside the field and a click there gives the index
//! back, with or without an icon, at 1x and 2x, so a caret moved into the
//! part a long text's tail hides is scrolled into view.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::Field;

use super::canvas::Canvas;
use super::field_pick::{caret_x, typing};
use super::samples::LONG;
use super::serial::serial;

const PLAIN: &str = "The quick brown fox jumps over the lazy dog, and then back again";

#[test]
fn field_keeps_every_caret_of_a_long_text_in_view() {
    let _g = serial();
    for q in [4, 8] {
        set_scale(q);
        for run in [PLAIN, LONG] {
            for f in [Field::new(run), Field::new(run).icon(IconId::Files)] {
                let (f, at) = typing(f);
                let ends = run.char_indices().map(|(i, _)| i).chain([run.len()]);
                for c in ends {
                    let x = caret_x(f, at, c);
                    assert!(x > at.x && x < at.x + at.w, "{run:?} caret {c} outside at {q}/4");
                    let back = f.caret(Some(c)).caret_at(at, x as i32);
                    assert_eq!(back, c, "{run:?} caret {c} at {q}/4");
                }
            }
        }
    }
}

#[test]
fn field_selection_runs_on_past_a_scrolled_caret() {
    let _g = serial();
    for q in [4, 8] {
        set_scale(q);
        let (f, at) = typing(Field::new(PLAIN).caret(Some(4)));
        let base = Canvas::new(at, 0xff10_2030);
        let lit = base.clone().painted(|fb| f.selection(Some((PLAIN.len(), 4))).draw(fb, at));
        let px = lit.diff(&base.painted(|fb| f.draw(fb, at)));
        let l = caret_x(f, at, 4);
        assert!(px.iter().any(|&(x, _)| x > l + at.w / 2), "selection stops short at {q}/4");
        assert!(px.iter().all(|&(x, _)| x >= l && x < at.x + at.w), "selection strays at {q}/4");
    }
}
