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
use nonos_toolkit::widgets::{text, Card};

use super::geom::{collapsed, storage};
use crate::catalog::scan::MAX_ENTRIES;
use crate::ui::format::count;
use crate::ui::text::body_px;

pub fn paint_storage(fb: &mut PaintBuffer, w: u32, h: u32, items: usize) {
    if collapsed(w) {
        return;
    }
    let (r, theme, s) = (storage(w, h), roles(), scale());
    Card::new().draw(fb, r);
    let (inner, x) = (r.w.saturating_sub(s.px(24)), r.x + s.px(12));
    text::left(fb, x, (r.y + s.px(9)) as i32, "Library", theme.text_2, body_px());
    /* A count, not a meter: the bar drawn here was the list against the
     * scan's 256-entry cap, which read like a disk filling up. */
    let label = if items >= MAX_ENTRIES {
        alloc::format!("First {MAX_ENTRIES} videos")
    } else {
        count(items, "video", "videos")
    };
    let shown = text::truncate(&label, inner, body_px());
    text::left(fb, x, (r.y + s.px(36)) as i32, &shown, theme.muted, body_px());
}
