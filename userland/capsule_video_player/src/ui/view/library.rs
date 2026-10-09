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

use super::grid::{paint_grid, paint_list};
use crate::app::state::VideoApp;
use crate::catalog::says::library_unavailable;
use crate::ui::format::count;
use crate::ui::icon;
use crate::ui::layout::Rect;
use crate::ui::widget::empty::paint_empty;
use crate::ui::widget::section::paint_head;

const HEAD: u32 = 34;

/// Where the videos are drawn under the page heading. The click test reads
/// the same rect, so a click lands on the tile or row painted under it.
pub fn area(body: Rect) -> Rect {
    Rect { x: body.x, y: body.y + HEAD, w: body.w, h: body.h.saturating_sub(HEAD) }
}

pub fn paint(fb: &mut PaintBuffer, app: &VideoApp, body: Rect) {
    if app.browse.items.is_empty() {
        let (head, note) = library_unavailable(app.browse.scanned, app.browse.scan_error)
            .unwrap_or((
                "Your library is empty",
                "Put Motion-JPEG .avi files in /, Movies, Series, Downloads or Clips",
            ));
        paint_empty(fb, body, icon::nav::library, head, note);
        return;
    }
    if app.browse.is_empty() {
        paint_empty(
            fb,
            body,
            icon::ui::search,
            "No matches",
            "No video in your library matches that search",
        );
        return;
    }
    let tally = count(app.browse.len(), "video", "videos");
    paint_head(fb, body.x, body.y, body.w, "All Videos", &tally);
    let rest = area(body);
    if app.browse.grid {
        paint_grid(fb, &app.browse, rest);
    } else {
        paint_list(fb, &app.browse, rest);
    }
}
