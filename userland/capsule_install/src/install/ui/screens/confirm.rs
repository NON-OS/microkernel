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

//! The last stop before the erase: the disk named again with what is on it,
//! everything the install erases and writes with its size, and the word to
//! type. Enter does nothing until the word matches, and nothing at all for
//! a disk that cannot take NONOS.

use nonos_app_skeleton::PaintBuffer;

use super::confirm_rows::written;
use super::confirm_warn::warn;
use super::confirm_word::word_field;
use crate::install::format::bytes;
use crate::install::state::State;
use crate::install::ui::frame::Body;
use crate::install::ui::metrics::confirm_stack;
use crate::install::ui::widgets::{card, kv};
use crate::install::ui::{text, theme};

pub fn paint(state: &State, fb: &mut PaintBuffer, b: Body) {
    let Some(d) = state.selected_disk() else { return };
    let m = &b.m;
    let inner = card(fb, m, b.x, b.y, b.w, m.card_h(4), "this disk");
    let (x, w) = (b.x + m.inset, b.w - 2 * m.inset);
    let model = d.identity.map(|i| alloc::string::String::from(i.model_str()));
    let mut r = kv(fb, m, x, inner, w, "disk", model.as_deref().unwrap_or(d.label()), false);
    let size = alloc::format!("{}, {}", bytes(d.bytes()), d.label());
    r = kv(fb, m, x, r, w, "size", &size, false);
    r = kv(fb, m, x, r, w, "holds now", d.contents.text(), false);
    warn(fb, m, x, r, d.contents);

    let plan_at = confirm_stack(m, 0)[0];
    let rows = written(state, fb, m, b.x, b.y + plan_at, b.w);
    let [_, ask_at, field_at, _] = confirm_stack(m, rows);
    // With no plan yet there is no card, and the line takes its place.
    let y = b.y + if rows == 0 { plan_at } else { ask_at };
    if !matches!(state.prepared, Some(Ok(_))) {
        text::line(fb, b.x, y, "Press Esc to choose another disk.", theme::MUTED, m.body_px);
        return;
    }
    let ask = "Type the word below, then press Enter to erase it and install.";
    text::line(fb, b.x, y, ask, theme::FOREGROUND, m.body_px);
    word_field(state, fb, m, b.x, b.y + field_at, &d.confirm_word());
}
