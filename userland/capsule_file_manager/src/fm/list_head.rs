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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw as paint, text, Button, Kind, Rect, Separator};

use super::layout::{font, hair, px};
use super::list_cols::{cols, head_h, HEAD_PX};
use super::state::{SortMode, State};
use super::target::Target;
use super::target_at::look;

const CARET: u32 = 10;
const CARET_GAP: u32 = 5;
const TEXT_DY: u32 = 4;

/// Which of the four columns the live sort mode belongs to. The sort model
/// cycles modes rather than directions, so the caret marks the active column
/// and says nothing about ascending or descending.
pub fn active_col(mode: SortMode) -> usize {
    match mode {
        SortMode::Name => 0,
        SortMode::Type => 1,
        SortMode::Size => 2,
        SortMode::Date => 3,
    }
}

/// The column header strip. Name is left-aligned over the name column and the
/// three meta columns right-align over the cells beneath them, so a label and
/// its values share one edge. Placement comes from `list_cols::cols`, the same
/// layout `browse_hit` tests a header click against, and each column is a
/// `Plain` button body, so it shows hover and press.
pub fn paint_head(state: &State, fb: &mut PaintBuffer, left: u32, cw: u32, y: u32) {
    let c = cols(left, cw);
    let on = active_col(state.sort_mode);
    let (dy, r, h, head_px) = (px(TEXT_DY), roles(), head_h(), font(HEAD_PX));
    let ty = y + dy;
    let (caret, caret_gap) = (px(CARET), px(CARET_GAP));
    for (i, col) in c.cols.iter().enumerate() {
        let body = Button::new("").kind(Kind::Plain).state(look(state, Target::Sort(i)));
        body.draw(fb, Rect { x: col.x, y, w: col.w, h });
        let lit = i == on;
        let ink = if lit { r.accent } else { r.muted }.argb();
        let tw = text::width(col.label, head_px);
        let tx = if i == 0 { col.x } else { (col.x + col.w).saturating_sub(tw) };
        let _ = fb.text_ttf(tx as i32, ty as i32, col.label, ink, head_px);
        if lit {
            let cx =
                if i == 0 { tx + tw + caret_gap } else { tx.saturating_sub(caret + caret_gap) };
            paint::icon(fb, IconId::ChevronRight, cx, ty + dy, caret, r.accent);
        }
    }
    let line = hair();
    Separator::new().draw(fb, Rect { x: left, y: y + h - line, w: cw, h: line });
}
