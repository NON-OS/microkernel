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

use super::metrics::{GAP_TIGHT, ICON_SM, PAD_TIGHT, PX_BODY, PX_LABEL, PX_STAT};
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
    let theme = roles();
    Card::new().state(look).draw(fb, r);
    let left = r.x + PAD_TIGHT;
    let max_w = r.w.saturating_sub(PAD_TIGHT * 2);
    let cap_h = line_height(PX_LABEL).max(1) as u32;
    let cap_top = r.y + PAD_TIGHT;
    let mark_y = cap_top + cap_h.saturating_sub(ICON_SM) / 2;
    draw(fb, icon, left, mark_y, ICON_SM, theme.muted.argb());
    let cap_w = max_w.saturating_sub(ICON_SM + GAP_TIGHT);
    let cap = text::truncate(as_str(caption), cap_w, PX_LABEL);
    text::left(fb, left + ICON_SM + GAP_TIGHT, cap_top as i32, &cap, theme.muted, PX_LABEL);
    let value_top = cap_top + cap_h + GAP_TIGHT;
    fb.text_ttf_mono(left as i32, value_top as i32, as_str(value), theme.text.argb(), PX_STAT);
    let sub_top = value_top + line_height(PX_STAT).max(1) as u32;
    let line = text::truncate(as_str(sub), max_w, PX_BODY);
    text::left(fb, left, sub_top as i32, &line, theme.text_2, PX_BODY);
}
