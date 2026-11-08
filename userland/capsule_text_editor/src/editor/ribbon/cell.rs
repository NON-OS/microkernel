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
use nonos_toolkit::widgets::Pointer;

use super::icons::chevron;
use super::items::RibbonItem;
use super::metrics::{Geom, RibbonCell, CHEVRON_W, PILL_PAD, SEP_GAP};
use super::snapshot::RibbonState;
use super::squares::square;
use crate::editor::layout::CHROME_PX;
use crate::editor::theme;

pub(super) fn paint_cell(
    fb: &mut PaintBuffer,
    c: &RibbonCell,
    g: &Geom,
    labels: &[&str; 3],
    st: &RibbonState,
    open: Option<usize>,
    pointer: &Pointer,
) {
    let th = theme::active();
    let w = c.x1 - c.x0;
    match c.item {
        RibbonItem::Pill(p) => {
            let fill = if open == Some(p) { th.tab_active_bg } else { th.tab_inactive_bg };
            fb.panel(c.x0, g.cy, w, g.ch, 5, fill, th.line);
            let x = (c.x0 + PILL_PAD) as i32;
            let _ = fb.text_ttf(x, g.ty, labels[p], th.foreground, CHROME_PX);
            chevron(fb, c.x1 - CHEVRON_W + 3, g.cy + g.ch / 2, th.muted);
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
        let c = theme::active().line;
        fb.fill_rect(x0 - SEP_GAP / 2, g.cy + 2, 1, g.ch.saturating_sub(4), c);
    }
}
