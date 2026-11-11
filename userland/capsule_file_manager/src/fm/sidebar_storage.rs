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

extern crate alloc;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw as paint, text, Card, Progress, Rect};

use super::icon_draw::draw;
use super::icon_path::Icon;
use super::sidebar_metrics::{line_box, rail_text_x, rail_w, ICON_S, RAIL_PAD, RAIL_X};
use super::sidebar_model::CARD_H;
use super::sidebar_usage::{fill_pct, occupancy};
use super::state::State;
use super::target::Target;
use super::target_at::look;

const TITLE_PX: f32 = 18.0;
const SUB_PX: f32 = 13.0;
const BAR_H: u32 = 6;
const CHEVRON: u32 = 14;
const DRIVE: &str = "NØNOS Drive";

/// The drive card pinned to the foot of the rail. Every figure on it comes from
/// the cached `vfs::usage` read taken on refresh, never from a call made inside
/// paint. `index` is the card's row in the sidebar's layout pass.
pub fn paint_storage(state: &State, fb: &mut PaintBuffer, index: usize, y: u32) {
    let (w, r) = (rail_w(), roles());
    let at = Rect { x: RAIL_X, y, w, h: CARD_H };
    Card::new().state(look(state, Target::Side(index))).draw(fb, at);
    let head = y + RAIL_PAD;
    draw(fb, Icon::Cube, RAIL_X + RAIL_PAD, head, ICON_S, r.accent.argb());
    let ty = head + ICON_S.saturating_sub(line_box(TITLE_PX)) / 2;
    let _ = fb.text_ttf(rail_text_x() as i32, ty as i32, DRIVE, r.text.argb(), TITLE_PX);
    let cx = RAIL_X + w.saturating_sub(RAIL_PAD + CHEVRON);
    let cy = head + ICON_S.saturating_sub(CHEVRON) / 2;
    paint::icon(fb, IconId::ChevronRight, cx, cy, CHEVRON, r.muted);
    let inner = w.saturating_sub(RAIL_PAD * 2);
    let bar_y = head + ICON_S + RAIL_PAD;
    let track = Rect { x: RAIL_X + RAIL_PAD, y: bar_y, w: inner, h: BAR_H };
    Progress::new(fill_pct(state).min(100), 100).draw(fb, track);
    let line = occupancy(state);
    let cut = text::truncate(line.as_str(), inner, SUB_PX);
    let ty = (bar_y + BAR_H + 6) as i32;
    text::left(fb, RAIL_X + RAIL_PAD, ty, &cut, r.muted, SUB_PX);
}
