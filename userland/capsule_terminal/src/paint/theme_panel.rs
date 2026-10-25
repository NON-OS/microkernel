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

//! The theme picker's panel: a row a theme, its name and six swatches of its
//! own colours (background, text, accent, path, success, failure), the one on
//! show marked; drawn where Tab's list goes, beside the prompt.

use nonos_app_skeleton::PaintBuffer;

use crate::layout::Layout;
use crate::paint::constants::TEXT_LEFT;
use crate::paint::line_text::text;
use crate::paint::metrics::Metrics;
use crate::paint::shade::elevate;
use crate::term::complete_list::top;
use crate::term::theme::profiles::{by_index, COUNT};
use crate::term::theme::types::Theme;
use crate::term::theme_pick::{Pick, NAMES};

/// Cells for the name column, and for each swatch.
const NAME_CELLS: u32 = 8;
const SWATCH_CELLS: u32 = 3;

pub fn draw(pick: &Pick, fb: &mut PaintBuffer, l: &Layout, m: Metrics, t: &Theme) {
    let lines = u32::from(COUNT);
    let y0 = top(l.input.y, m.lh, lines, l.body.y, l.footer.y);
    let x = l.input.x + TEXT_LEFT;
    let w = (2 + NAME_CELLS + 6 * SWATCH_CELLS) * m.adv;
    fb.fill_rect(x - 6, y0, w + 12, lines * m.lh, elevate(t.bg, 14));
    fb.fill_rect(x - 6, y0, 2, lines * m.lh, t.accent);
    for i in 0..COUNT {
        let y = y0 + u32::from(i) * m.lh;
        let on = i == pick.at;
        if on {
            fb.fill_rect(x - 4, y, w + 10, m.lh, elevate(t.bg, 26));
            text(fb, x, y, b">", t.accent, m.adv, m.px);
        }
        let name = NAMES.get(usize::from(i)).copied().unwrap_or(b"?");
        text(fb, x + 2 * m.adv, y, name, if on { t.accent } else { t.fg }, m.adv, m.px);
        let th = by_index(i);
        let sw = [th.bg, th.fg, th.accent, th.path, th.ok, th.err];
        for (k, c) in sw.iter().enumerate() {
            let sx = x + (2 + NAME_CELLS + k as u32 * SWATCH_CELLS) * m.adv;
            fb.fill_rect(sx, y + 3, (SWATCH_CELLS - 1) * m.adv, m.lh - 6, t.chrome_edge);
            fb.fill_rect(sx + 1, y + 4, (SWATCH_CELLS - 1) * m.adv - 2, m.lh - 8, c | 0xFF00_0000);
        }
    }
}
