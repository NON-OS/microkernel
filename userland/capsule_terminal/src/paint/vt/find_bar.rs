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

//! The search bar, drawn where the prompt goes while a search is open.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::{Align, Field, Ink, Label, Rect, State};

use crate::paint::find_view::{hint, view, CASE, PLACEHOLDER};
use crate::term::select::Find;

/// The query in a focused field with the caret at its end, the hint muted
/// beside it in the hint bar, and the case mark in the accent.
pub fn draw_find_bar(f: &Find, fb: &mut PaintBuffer, input: Rect, footer: Rect) {
    let v = view(input, footer, f.case);
    let focused = State { focused: true, ..State::default() };
    let field = Field::new(&f.query).placeholder(PLACEHOLDER).caret(Some(f.query.len()));
    field.state(focused).draw(fb, v.field);
    Label::new(hint(f)).ink(Ink::Muted).draw(fb, v.hint);
    if let Some(at) = v.case {
        Label::new(CASE).ink(Ink::Accent).align(Align::Right).draw(fb, at);
    }
}
