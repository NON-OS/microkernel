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

use nonos_toolkit::widgets::{Field as Entry, State};

use crate::browser::shell::settings_ui::SettingsUi;
use crate::browser::ui::rect::{area, Rect};
use crate::browser::ui::settings_geom::Field;

/// Field `f` of the page: its text, its hint while empty, and while it
/// has the keyboard the accent edge, the caret and the selection.
pub fn entry(ui: &SettingsUi, f: Field) -> Entry<'_> {
    let (ed, hint) = match f {
        Field::Proxy => (&ui.proxy, "127.0.0.1:9050"),
        Field::Search => (&ui.search, "https://example.org/search?q=%s"),
    };
    let state = State { focused: ui.focus == Some(f), ..State::default() };
    let sel = Some((ed.anchor, ed.caret));
    Entry::new(&ed.text).placeholder(hint).caret(Some(ed.caret)).selection(sel).state(state)
}

/// The byte index into field `f`'s text where a click at `x` puts the
/// caret, with the field drawn in `r` as it stands: the boundary it draws
/// nearest to the click.
pub fn caret_at(ui: &SettingsUi, f: Field, r: Rect, x: i32) -> usize {
    entry(ui, f).caret_at(area(r), x)
}
