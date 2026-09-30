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

//! The word field: the typed text as typed, its border green when it
//! matches and amber while it does not, and the word beside it.

use nonos_app_skeleton::PaintBuffer;

use crate::install::state::State;
use crate::install::ui::metrics::{BODY_PX, MONO_PX, RADIUS};
use crate::install::ui::{text, theme};

pub fn word_field(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, word: &str) {
    let typed = core::str::from_utf8(&state.typed).unwrap_or("");
    let border = if typed == word {
        theme::OK
    } else if typed.is_empty() {
        theme::CARD_BORDER
    } else {
        theme::WARN
    };
    fb.panel(x, y, 300, 44, RADIUS, theme::CARD_BG, border);
    text::mono(fb, x + 16, y + 12, typed, theme::TITLE, MONO_PX);
    /*
     * The caret: a thin bar after the text, so it is clear the field is live.
     */
    let caret_x = x + 16 + text::width(fb, typed, MONO_PX) + 2;
    fb.fill_rect(caret_x, y + 11, 2, 22, theme::ACCENT);
    let hint = alloc::format!("the word is  {word}");
    text::line(fb, x + 320, y + 12, &hint, theme::MUTED, BODY_PX);
}
