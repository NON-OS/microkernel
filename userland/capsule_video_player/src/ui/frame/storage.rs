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

use super::geom::{collapsed, storage};
use crate::catalog::scan::MAX_ENTRIES;
use crate::ui::fit::fit;
use crate::ui::format::count;
use crate::ui::paint::rrect;
use crate::ui::text::BODY_PX;
use crate::ui::theme;

pub fn paint_storage(fb: &mut PaintBuffer, w: u32, h: u32, items: usize) {
    if collapsed(w) {
        return;
    }
    let r = storage(w, h);
    rrect::panel(fb, r.x, r.y, r.w, r.h, 10, theme::PANEL, theme::BORDER_SOFT);
    let inner = r.w.saturating_sub(24);
    fb.text_ttf((r.x + 12) as i32, (r.y + 9) as i32, "Library", theme::TEXT_DIM, BODY_PX);
    // A count, not a meter: the bar drawn here was the list against the
    // scan's 256-entry cap, which read like a disk filling up.
    let label = if items >= MAX_ENTRIES {
        alloc::format!("First {MAX_ENTRIES} videos")
    } else {
        count(items, "video", "videos")
    };
    let shown = fit(&label, inner, BODY_PX);
    fb.text_ttf((r.x + 12) as i32, (r.y + 36) as i32, shown, theme::TEXT_MUTED, BODY_PX);
}
