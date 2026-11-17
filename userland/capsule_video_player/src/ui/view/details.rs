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

use nonos_app_skeleton::paint::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Ink, Label, Pill, Progress, Rect};

use super::empty::paint_empty;
use super::page_geom::{strip, tabs, STRIP_H, TABS};
use crate::app::state::VideoApp;
use crate::catalog::entry::parent_dir;
use crate::ui::format::{duration, resolution, size};
use crate::ui::icon;
use crate::ui::target::Target;
use crate::ui::text::{body_px, head_px, line_h};
use crate::ui::widget::poster::paint_poster;

const POSTER_W: u32 = 260;
const POSTER_H: u32 = 150;
const ROW_H: u32 = 30;
const PILL_GAP: u32 = 8;

/// Pills for `labels` from `x`, `y`, wrapping to a new line past `w`; the y
/// under the last line.
fn pills(fb: &mut PaintBuffer, (x, y, w): (u32, u32, u32), labels: &[&str]) -> u32 {
    let (mut pen, mut row, h) = (x, y, Pill::new("").size().1);
    let gap = scale().px(PILL_GAP);
    for label in labels {
        let pill = Pill::new(label);
        let need = pill.size().0;
        if pen > x && pen + need > x + w {
            (pen, row) = (x, row + h + gap);
        }
        pill.draw(fb, Rect { x: pen, y: row, w: need, h });
        pen += need + gap;
    }
    row + h
}

pub fn paint(fb: &mut PaintBuffer, app: &VideoApp, outer: Rect) {
    let hovered = (0..TABS.len()).find(|&i| app.look(Target::Tab(i)).hovered);
    tabs().hovered(hovered).draw(fb, strip(outer));
    let s = scale();
    let below = s.px(STRIP_H) + s.px(12);
    let body = Rect { y: outer.y + below, h: outer.h.saturating_sub(below), ..outer };
    /* The video the player has open, which a handed-over file or a search
     * since can make other than the library's selected row. */
    let item = match app.playing_item() {
        Some(item) => item,
        None => {
            paint_empty(
                fb,
                body,
                icon::ui::info,
                "Nothing playing",
                "Open a video from your library to see its details",
            );
            return;
        }
    };

    let poster_w = s.px(POSTER_W);
    let poster = Rect { x: body.x, y: body.y, w: poster_w, h: s.px(POSTER_H) };
    paint_poster(fb, poster, item.kind, item.thumb.as_deref());
    let bar = Rect { x: body.x, y: poster.y + poster.h + s.px(14), w: poster_w, h: s.px(6) };
    let theme = roles();
    Progress::new(item.permille(), 1000).draw(fb, bar);
    let resume = duration(item.resume_ms);
    let total = duration(item.duration_ms);
    let line_y = (bar.y + bar.h + s.px(10)) as i32;
    text::left(fb, body.x, line_y, &resume, theme.text_2, body_px());
    text::right(fb, body.x + poster_w, line_y, &total, theme.muted, body_px());

    let col_x = body.x + poster_w + s.px(24);
    let col_w = (body.x + body.w).saturating_sub(col_x);
    let title = text::truncate(item.title(), col_w, head_px());
    text::left(fb, col_x, body.y as i32, &title, theme.text, head_px());
    let tags_y = body.y + s.px(34);
    let kind_label = item.kind.label();
    let state_label = if item.decodable() { "Playable" } else { "Not decodable" };
    let after_chips = pills(fb, (col_x, tags_y, col_w), &[kind_label, state_label]);

    let info_y = after_chips + s.px(20);
    let label = Rect { x: col_x, y: info_y, w: col_w, h: line_h() };
    Label::new("FILE INFORMATION").ink(Ink::Muted).draw(fb, label);
    let size_text = size(item.size);
    let resolution_text = resolution(item.width, item.height);
    let rows: [(&str, &str); 4] = [
        ("Name", &item.name),
        ("Location", parent_dir(&item.path)),
        ("Size", &size_text),
        ("Resolution", &resolution_text),
    ];
    let value_x = col_x + s.px(150);
    let value_w = (col_x + col_w).saturating_sub(value_x);
    for (i, (key, value)) in rows.iter().enumerate() {
        let y = (info_y + s.px(26) + i as u32 * s.px(ROW_H)) as i32;
        text::left(fb, col_x, y, key, theme.muted, body_px());
        let shown = text::truncate(value, value_w, body_px());
        text::left(fb, value_x, y, &shown, theme.text, body_px());
    }
}
