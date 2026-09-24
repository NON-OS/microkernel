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

//! The last stop before the erase: the disk named again, what is on it,
//! and the word to type. The typed text is shown as typed; the word turns
//! green when it matches and Enter does nothing until it does.

use nonos_app_skeleton::PaintBuffer;

use super::confirm_warn::warn;

use crate::install::format::bytes;
use crate::install::state::State;
use crate::install::ui::frame::Body;
use crate::install::ui::metrics::{BODY_PX, LINE_H, MONO_PX, RADIUS};
use crate::install::ui::widgets::{card, kv};
use crate::install::ui::{text, theme};

pub fn paint(state: &State, fb: &mut PaintBuffer, b: Body) {
    let Some(d) = state.selected_disk() else { return };
    let inner = card(fb, b.x, b.y, b.w, 4 * LINE_H + 40, "this disk");
    let (x, w) = (b.x + 16, b.w - 32);
    let model = d.identity.map(|i| alloc::string::String::from(i.model_str()));
    let mut r = kv(fb, x, inner, w, "disk", model.as_deref().unwrap_or(d.label()), false);
    r = kv(fb, x, r, w, "size", &bytes(d.bytes()), false);
    r = kv(fb, x, r, w, "holds now", d.contents.text(), false);
    warn(fb, x, r, d.contents);

    let y = b.y + 4 * LINE_H + 64;
    text::line(
        fb,
        b.x,
        y,
        "Type the word below, then press Enter to erase it and install.",
        theme::FOREGROUND,
        BODY_PX,
    );
    let word = d.confirm_word();
    let typed = core::str::from_utf8(&state.typed).unwrap_or("");
    let matched = typed == word;

    let field_y = y + LINE_H + 12;
    let border = if matched {
        theme::OK
    } else if typed.is_empty() {
        theme::CARD_BORDER
    } else {
        theme::WARN
    };
    fb.panel(b.x, field_y, 300, 44, RADIUS, theme::CARD_BG, border);
    text::mono(fb, b.x + 16, field_y + 12, typed, theme::TITLE, MONO_PX);
    // The caret: a thin bar after the text, so it is clear the field is live.
    let caret_x = b.x + 16 + text::width(fb, typed, MONO_PX) + 2;
    fb.fill_rect(caret_x, field_y + 11, 2, 22, theme::ACCENT);

    let hint = alloc::format!("the word is  {word}");
    text::line(fb, b.x + 320, field_y + 12, &hint, theme::MUTED, BODY_PX);
}
