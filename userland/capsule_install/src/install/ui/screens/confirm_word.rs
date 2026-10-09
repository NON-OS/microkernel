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
use crate::install::ui::metrics::Metrics;
use crate::install::ui::text::top_of;
use crate::install::ui::{text, theme};

pub fn word_field(state: &State, fb: &mut PaintBuffer, m: &Metrics, x: u32, y: u32, word: &str) {
    let typed = core::str::from_utf8(&state.typed).unwrap_or("");
    let border = if typed == word {
        theme::OK
    } else if typed.is_empty() {
        theme::CARD_BORDER
    } else {
        theme::WARN
    };
    let (fw, fh) = (m.field_w, m.field_h);
    fb.panel(x, y, fw, fh, m.radius, theme::CARD_BG, border);
    let top = top_of(y, fh, m.mono_px);
    text::mono(fb, x + m.inset, top, typed, theme::TITLE, m.mono_px);
    /*
     * The caret: a thin bar after the text, so it is clear the field is live.
     */
    let caret_x = x + m.inset + text::width(fb, typed, m.mono_px) + m.scale.px(2);
    let caret_h = fh.saturating_sub(2 * m.unit + m.unit / 2).max(m.unit);
    fb.fill_rect(caret_x, y + (fh - caret_h) / 2, m.scale.px(2), caret_h, theme::ACCENT);
    let hint = alloc::format!("the word is  {word}");
    let hx = x + fw + m.inset;
    text::line(fb, hx, top_of(y, fh, m.body_px), &hint, theme::MUTED, m.body_px);
}
