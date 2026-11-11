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

//! One ribbon control: a value pill, or a toggle or paragraph icon drawn as
//! the shared button, filled while the caret's run or paragraph carries it.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::icons::{draw, IconId};
use nonos_toolkit::widgets::Pointer;

use super::items::RibbonItem;
use super::metrics::{lengths, Geom, RibbonCell};
use super::snapshot::RibbonState;
use super::squares::square;
use crate::editor::layout::chrome_px;
use crate::editor::theme;

const PILL_R: u32 = 5;

pub(super) fn paint_cell(
    fb: &mut PaintBuffer,
    c: &RibbonCell,
    g: &Geom,
    labels: &[&str; 3],
    st: &RibbonState,
    open: Option<usize>,
    pointer: &Pointer,
) {
    let (th, l, k) = (theme::active(), lengths(), scale());
    let w = c.x1 - c.x0;
    match c.item {
        RibbonItem::Pill(p) => {
            let fill = if open == Some(p) { th.tab_active_bg } else { th.tab_inactive_bg };
            fb.panel(c.x0, g.cy, w, g.ch, k.px(PILL_R), fill, th.line);
            let x = (c.x0 + l.pill_pad) as i32;
            let _ = fb.text_ttf(x, g.ty, labels[p], th.foreground, chrome_px());
            let y = (g.cy + g.ch / 2).saturating_sub(l.chevron / 2);
            let cx = c.x1 - l.chevron_w + k.px(1);
            draw(fb, IconId::ChevronDown, cx, y, l.chevron, th.muted);
        }
        RibbonItem::Toggle(t) | RibbonItem::Icon(t) => {
            separator(fb, c.x0, g, t == 0);
            if let Some(sq) = square(c, st) {
                sq.draw(fb, pointer);
            }
        }
    }
}

fn separator(fb: &mut PaintBuffer, x0: u32, g: &Geom, first: bool) {
    if first {
        let (c, k) = (theme::active().line, scale());
        let h = g.ch.saturating_sub(k.px(4));
        fb.fill_rect(x0 - lengths().sep_gap / 2, g.cy + k.px(2), k.hair(), h, c);
    }
}
