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
//! Field: a click puts the caret on the boundary the painter draws it at, at
//! 1x, 1.25x and 2x, for plain, multi-byte and scrolled text.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::{Field, Rect, State};

use super::canvas::Canvas;
use super::samples::{LONG, RUNS};
use super::serial::{serial, SCALES};

/// `f` with the keyboard, drawn in its own size.
pub(crate) fn typing(f: Field<'static>) -> (Field<'static>, Rect) {
    let f = f.state(State { focused: true, ..State::default() });
    (f, Rect { x: 24, y: 24, w: f.size().0, h: f.size().1 })
}

/// The x the painter draws `f`'s caret at, at byte `c`.
pub(crate) fn caret_x(f: Field<'static>, at: Rect, c: usize) -> u32 {
    let base = Canvas::new(at, 0xff10_2030);
    let with = base.clone().painted(|fb| f.caret(Some(c)).draw(fb, at));
    let without = base.painted(|fb| f.caret(None).draw(fb, at));
    with.diff(&without).into_iter().map(|p| p.0).min().expect("caret drawn")
}

fn ends(s: &str) -> Vec<usize> {
    s.char_indices().map(|(i, _)| i).chain([s.len()]).collect()
}

#[test]
fn field_caret_at_lands_where_the_caret_is_drawn() {
    let _g = serial();
    for q in SCALES {
        set_scale(q);
        for run in RUNS.into_iter().filter(|r| r.len() < 30) {
            let (f, at) = typing(Field::new(run));
            for c in ends(run) {
                assert_eq!(f.caret_at(at, caret_x(f, at, c) as i32), c, "{run:?} {c} {q}/4");
            }
            assert_eq!((f.caret_at(at, 0), f.caret_at(at, i32::MAX)), (0, run.len()), "{q}/4");
        }
    }
}

#[test]
fn field_caret_at_follows_the_scrolled_tail() {
    let _g = serial();
    for q in SCALES {
        set_scale(q);
        for f in [Field::new(LONG), Field::new(LONG).icon(nonos_toolkit::icons::IconId::Files)] {
            let (f, at) = typing(f);
            let head = caret_x(f, at, 0);
            for c in ends(LONG) {
                let (x, k) = (caret_x(f, at, c), f.caret_at(at, caret_x(f, at, c) as i32));
                assert!(LONG.is_char_boundary(k), "{k} at {q}/4");
                assert_eq!(caret_x(f, at, k), x, "{c} round trip at {q}/4");
                assert!(x == head || k == c, "{c} clicked back as {k} at {q}/4");
            }
        }
    }
}
