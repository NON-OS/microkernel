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

//! Tab's candidates, drawn as a panel beside the prompt (`complete_list`), or
//! the theme picker's in their place while it is open.

use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;

use crate::paint::constants::TEXT_LEFT;
use crate::paint::line_text::text;
use crate::paint::metrics::Metrics;
use crate::paint::shade::elevate;
use crate::layout::Layout;
use crate::term::complete_list::{cell, grid, label, top};
use crate::term::state::State;
use crate::term::theme::types::Theme;
use crate::term::util::format_u64;

pub fn draw_completions(state: &State, fb: &mut PaintBuffer, l: &Layout, m: Metrics, t: &Theme) {
    if let Some(pick) = &state.theme_pick {
        return super::theme_panel::draw(pick, fb, l, m, t);
    }
    if state.completions.is_empty() {
        return;
    }
    let labels: Vec<&[u8]> = state.completions.iter().map(|c| label(c)).collect();
    let widths: Vec<usize> = labels.iter().map(|s| cells(s)).collect();
    let room = (l.input.w.saturating_sub(TEXT_LEFT * 2) / m.adv.max(1)) as usize;
    let g = grid(&widths, room);
    let more = labels.len() - g.shown;
    let lines = g.rows as u32 + u32::from(more > 0);
    let y = top(l.input.y, m.lh, lines, l.body.y, l.footer.y);
    let x = l.input.x + TEXT_LEFT;
    let w = ((g.cols * g.width).min(room) as u32 * m.adv).min(l.input.w - TEXT_LEFT);
    fb.fill_rect(x - 6, y, w + 12, lines * m.lh, elevate(t.bg, 14));
    fb.fill_rect(x - 6, y, 2, lines * m.lh, t.accent);
    for (i, name) in labels.iter().take(g.shown).enumerate() {
        let (c, r) = cell(&g, i);
        let tone = if name.ends_with(b"/") { t.path } else { t.fg };
        let cx = x + (c * g.width) as u32 * m.adv;
        text(fb, cx, y + r as u32 * m.lh, name, tone, m.adv, m.px);
    }
    if more > 0 {
        let mut buf = [0u8; 24];
        buf[..1].copy_from_slice(b"+");
        let n = 1 + format_u64(more as u64, &mut buf[1..12]);
        buf[n..n + 5].copy_from_slice(b" more");
        text(fb, x, y + g.rows as u32 * m.lh, &buf[..n + 5], t.dim, m.adv, m.px);
    }
}

fn cells(s: &[u8]) -> usize {
    s.iter().filter(|b| **b & 0xC0 != 0x80).count()
}
