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
use crate::install::ui::metrics::{BODY_PX, LINE_H};
use crate::install::ui::widgets::{card, kv};
use crate::install::ui::{text, theme};

pub fn paint(state: &State, fb: &mut PaintBuffer, b: Body) {
    let Some(d) = state.selected_disk() else { return };
    let inner = card(fb, b.x, b.y, b.w, 4 * LINE_H + 40, "this disk");
    let (x, w) = (b.x + 16, b.w - 32);
    let model = d.identity.map(|i| alloc::string::String::from(i.model_str()));
    let mut r = kv(fb, x, inner, w, "disk", model.as_deref().unwrap_or(d.label()), false);
    let size = alloc::format!("{}, {}", bytes(d.bytes()), d.label());
    r = kv(fb, x, r, w, "size", &size, false);
    r = kv(fb, x, r, w, "holds now", d.contents.text(), false);
    warn(fb, x, r, d.contents);

    let y = written(state, fb, b.x, b.y + 4 * LINE_H + 52, b.w) + 12;
    if !matches!(state.prepared, Some(Ok(_))) {
        text::line(fb, b.x, y, "Press Esc to choose another disk.", theme::MUTED, BODY_PX);
        return;
    }
    let ask = "Type the word below, then press Enter to erase it and install.";
    text::line(fb, b.x, y, ask, theme::FOREGROUND, BODY_PX);
    word_field(state, fb, b.x, y + LINE_H + 4, &d.confirm_word());
}
