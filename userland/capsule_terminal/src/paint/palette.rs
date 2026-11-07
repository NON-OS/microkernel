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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Field, Rect, State};

use super::palette_view::{items, lit, matches, view};
use crate::palette::{Entry, Palette};

const PLACEHOLDER: &str = "Run a command, open a session or a project";

/// The first overlay layer in this window: a scrim over the body, the query
/// field holding the keyboard, and the shared menu of what it matches, with
/// the selection washed and a held row filled.
pub fn draw(fb: &mut PaintBuffer, body: Rect, pal: &Palette, entries: &[Entry]) {
    fb.blend_rect(body.x, body.y, body.w, body.h, roles().scrim.argb());
    let m = matches(entries, pal.needle());
    let rows = items(&m);
    let v = view(body, &rows);
    let text = core::str::from_utf8(pal.needle()).unwrap_or("");
    let focused = State { focused: true, ..State::default() };
    let field = Field::new(text).placeholder(PLACEHOLDER).caret(Some(text.len()));
    field.state(focused).draw(fb, v.field);
    if rows.is_empty() {
        return;
    }
    let (hovered, pressed) = lit(&v, &pal.ptr, pal.sel);
    v.menu.hovered(hovered).pressed(pressed).draw(fb, v.at);
}
