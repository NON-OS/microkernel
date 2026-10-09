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

use crate::layout::Rect;
use crate::paint::constants::TEXT_LEFT;
use crate::paint::metrics::Metrics;
use crate::paint::shade::elevate;
use crate::term::select::Find;
use crate::term::theme::types::Theme;

pub fn draw_find_bar(f: &Find, fb: &mut PaintBuffer, r: Rect, m: Metrics, t: &Theme) {
    let bar_x = r.x + TEXT_LEFT / 2;
    let bar_y = r.y.saturating_sub(3);
    fb.fill_rect(bar_x, bar_y, r.w.saturating_sub(TEXT_LEFT), m.lh + 4, elevate(t.bg, 12));
    fb.fill_rect(bar_x, bar_y, 2, m.lh + 4, t.run);
    let x = (r.x + TEXT_LEFT) as i32;
    let label = "find ";
    let _ = fb.text_ttf_mono(x, r.y as i32, label, t.dim, m.px);
    let qx = x + (label.len() as u32 * m.adv) as i32;
    let _ = fb.text_ttf_mono(qx, r.y as i32, &f.query, t.fg, m.px);
    let state = match (f.query.is_empty(), f.hit.is_some()) {
        (true, _) => "type to search history",
        (false, true) => "Enter older, Shift+Enter newer, Esc close",
        (false, false) => "no match",
    };
    let case = if f.case { "  case" } else { "" };
    let hint_x = qx + ((f.query.chars().count() as u32 + 3) * m.adv) as i32;
    let _ = fb.text_ttf_mono(hint_x, r.y as i32, state, t.dim, m.px);
    if !case.is_empty() {
        let cx = hint_x + (state.len() as u32 * m.adv) as i32;
        let _ = fb.text_ttf_mono(cx, r.y as i32, case, t.accent, m.px);
    }
}
