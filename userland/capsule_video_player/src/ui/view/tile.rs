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
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Card, Progress, Rect, State};

use crate::catalog::media::MediaItem;
use crate::ui::format::duration;
use crate::ui::text::BODY_PX;
use crate::ui::widget::poster::paint_poster;

const ART_H: u32 = 118;
const BAR_H: u32 = 4;

pub fn paint_tile(fb: &mut PaintBuffer, r: Rect, item: &MediaItem, look: State) {
    let theme = roles();
    Card::new().state(look).draw(fb, r);
    let art = Rect { x: r.x + 8, y: r.y + 8, w: r.w - 16, h: ART_H };
    paint_poster(fb, art, item.kind, item.thumb.as_deref());
    let permille = item.permille();
    if permille > 0 {
        let bar = Rect { y: art.y + art.h - BAR_H, h: BAR_H, ..art };
        Progress::new(permille, 1000).draw(fb, bar);
    }
    let room = r.w - 24;
    let title = text::truncate(item.title(), room, BODY_PX);
    text::left(fb, r.x + 12, (art.y + art.h + 12) as i32, &title, theme.text, BODY_PX);
    let sub = text::truncate(&duration(item.duration_ms), room, BODY_PX);
    text::left(fb, r.x + 12, (art.y + art.h + 34) as i32, &sub, theme.muted, BODY_PX);
}
