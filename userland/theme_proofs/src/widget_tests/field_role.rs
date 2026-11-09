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

//! Field: at Body and Title, every caret of a plain, multi-byte and long text
//! is drawn inside the field and a click there gives it back, at 1x and 2x,
//! and a Title field's text, caret and placeholder stay in the box.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::design::text_role::TextRole;
use nonos_toolkit::widgets::Field;

use super::bounds::assert_bounded;
use super::field_pick::{caret_x, typing};
use super::samples::LONG;
use super::serial::serial;

/// The roles the proofs draw a field at.
pub(crate) const ROLES: [TextRole; 2] = [TextRole::Body, TextRole::Title];
/// A plain, a multi-byte and a long text.
pub(crate) const RUNS: [&str; 3] = ["Saved tabs", "Grüße — 日本語", LONG];

#[test]
fn field_role_keeps_every_caret_where_a_click_finds_it() {
    let _g = serial();
    for (q, role) in [4, 8].into_iter().flat_map(|q| ROLES.map(|r| (q, r))) {
        set_scale(q);
        for run in RUNS {
            let (f, at) = typing(Field::new(run).role(role));
            for c in run.char_indices().map(|(i, _)| i).chain([run.len()]) {
                let x = caret_x(f, at, c);
                assert!(x > at.x && x < at.x + at.w, "{role:?} {run:?} {c} outside at {q}/4");
                let back = f.caret(Some(c)).caret_at(at, x as i32);
                assert_eq!(back, c, "{role:?} {run:?} caret {c} at {q}/4");
            }
        }
    }
}

#[test]
fn field_role_paints_inside_its_box() {
    let _g = serial();
    let title = |f: Field<'static>| f.role(TextRole::Title);
    let mut all = vec![("placeholder".to_string(), title(Field::new("").placeholder(LONG)))];
    for run in RUNS {
        all.push((format!("{run:?}"), title(Field::new(run).caret(Some(run.len())))));
    }
    assert_bounded(&all);
}
