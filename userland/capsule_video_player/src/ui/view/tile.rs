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

//! One video in the grid: a shared Card holding its poster, how far it was
//! watched as a shared Progress bar, its title and its length.

use nonos_app_skeleton::paint::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Card, Progress, Rect, State};

use crate::catalog::media::MediaItem;
use crate::ui::format::duration;
use crate::ui::text::body_px;
use crate::ui::widget::poster::paint_poster;

const ART_H: u32 = 118;
const BAR_H: u32 = 4;

pub fn paint_tile(fb: &mut PaintBuffer, r: Rect, item: &MediaItem, look: State) {
    let (theme, s) = (roles(), scale());
    Card::new().state(look).draw(fb, r);
    let inset = s.px(8);
    let art = Rect { x: r.x + inset, y: r.y + inset, w: r.w - inset * 2, h: s.px(ART_H) };
    paint_poster(fb, art, item.kind, item.thumb.as_deref());
    let permille = item.permille();
    if permille > 0 {
        let bar_h = s.px(BAR_H);
        let bar = Rect { y: art.y + art.h - bar_h, h: bar_h, ..art };
        Progress::new(permille, 1000).draw(fb, bar);
    }
    let (room, x, under) = (r.w - s.px(24), r.x + s.px(12), art.y + art.h);
    let title = text::truncate(item.title(), room, body_px());
    text::left(fb, x, (under + s.px(12)) as i32, &title, theme.text, body_px());
    let sub = text::truncate(&duration(item.duration_ms), room, body_px());
    text::left(fb, x, (under + s.px(34)) as i32, &sub, theme.muted, body_px());
}
