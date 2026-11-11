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
//! A figure on a shared card: a caption with its mark, the number under it
//! in the mono face, then a line saying what it counts. No shared widget draws
//! a large figure or a mono run, so this face stays Snake's own.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::font::ttf::line_height;
use nonos_toolkit::icons::{draw, IconId};
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Card, Rect, State};

use super::metrics::metrics;
use super::text::as_str;

/// Paints a card at `r` in the state `look`, then on it the caption after
/// its mark, `value` under it and `sub` below that. The caption and `sub`
/// are cut with an ellipsis to the card.
pub fn stat(
    fb: &mut PaintBuffer,
    r: Rect,
    look: State,
    icon: IconId,
    caption: &[u8],
    value: &[u8],
    sub: &[u8],
) {
    let (theme, m) = (roles(), metrics());
    Card::new().state(look).draw(fb, r);
    let left = r.x + m.pad_tight;
    let max_w = r.w.saturating_sub(m.pad_tight * 2);
    let cap_h = line_height(m.px_label).max(1) as u32;
    let cap_top = r.y + m.pad_tight;
    let mark_y = cap_top + cap_h.saturating_sub(m.icon_sm) / 2;
    draw(fb, icon, left, mark_y, m.icon_sm, theme.muted.argb());
    let cap_w = max_w.saturating_sub(m.icon_sm + m.gap_tight);
    let cap = text::truncate(as_str(caption), cap_w, m.px_label);
    text::left(fb, left + m.icon_sm + m.gap_tight, cap_top as i32, &cap, theme.muted, m.px_label);
    let value_top = cap_top + cap_h + m.gap_tight;
    fb.text_ttf_mono(left as i32, value_top as i32, as_str(value), theme.text.argb(), m.px_stat);
    let sub_top = value_top + line_height(m.px_stat).max(1) as u32;
    let line = text::truncate(as_str(sub), max_w, m.px_body);
    text::left(fb, left, sub_top as i32, &line, theme.text_2, m.px_body);
}
