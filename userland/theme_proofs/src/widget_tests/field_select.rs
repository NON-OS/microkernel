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
//! Field: a selection is drawn only between the carets at its two ends and
//! only while the field has the keyboard, and the text over it reads at
//! 4.5:1 in every scheme and high contrast.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::{Field, State};

use super::canvas::Canvas;
use super::field_pick::{caret_x, typing};
use super::hide::Hide;
use super::legible::{assert_legible_in, Off, Read};
use super::samples::LONG;
use super::serial::{serial, SCALES};
use super::states::looks;
use crate::wcag::BODY;

const TEXT: &str = "Grüße — 日本語";

fn wash(run: &'static str, sel: Option<(usize, usize)>) -> Vec<(u32, u32)> {
    let (f, at) = typing(Field::new(run));
    let base = Canvas::new(at, 0xff10_2030);
    let with = base.clone().painted(|fb| f.selection(sel).draw(fb, at));
    with.diff(&base.painted(|fb| f.draw(fb, at)))
}

#[test]
fn field_selection_lies_between_its_carets() {
    let _g = serial();
    let cases = [(TEXT, 0, 6), (TEXT, TEXT.len(), 4), (TEXT, 2, TEXT.len()), (LONG, 0, LONG.len())];
    for q in SCALES {
        set_scale(q);
        for (run, a, b) in cases {
            let (f, at) = typing(Field::new(run));
            let (l, r) = (caret_x(f, at, a.min(b)), caret_x(f, at, a.max(b)));
            let px = wash(run, Some((a, b)));
            assert!(!px.is_empty(), "{run:?} {a}..{b} at {q}/4 draws nothing");
            assert!(px.iter().all(|&(x, _)| x >= l && x < r), "{run:?} {a}..{b} at {q}/4");
            assert!(wash(run, Some((a, a))).is_empty(), "{run:?} empty at {q}/4");
        }
    }
}

#[test]
fn field_selection_needs_the_keyboard() {
    let _g = serial();
    let (f, at) = typing(Field::new(TEXT).selection(Some((0, TEXT.len()))));
    let base = Canvas::new(at, 0xff10_2030);
    for st in [State::default(), State { focused: true, disabled: true, ..State::default() }] {
        let plain = base.clone().painted(|fb| Field::new(TEXT).state(st).draw(fb, at));
        assert!(plain.diff(&base.clone().painted(|fb| f.state(st).draw(fb, at))).is_empty());
    }
}

#[test]
fn field_selected_text_reads_in_every_scheme() {
    let _g = serial();
    let on = Field::new(TEXT).selection(Some((0, TEXT.len())));
    let read = Read { what: "selected text", on, off: Off::Hide(Hide::Text(TEXT)), floor: BODY };
    assert_legible_in(&[read], || looks().filter(|s| s.focused));
}
