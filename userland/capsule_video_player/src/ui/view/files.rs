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

//! Folders: the roots the catalogue reads down the left, each with the number
//! of videos found in it, and the chosen folder's videos on the right with an
//! Open button. Where each part sits is in `rail_geom`, which the click and
//! the hover read too.

use nonos_app_skeleton::paint::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, text, Button, Ink, Kind, Label, NavItem, Rect};

use super::empty::paint_empty;
use super::grid::{paint_grid, paint_list};
use super::rail_geom::{folder_area, open_button, rail_row, RAIL_ROWS, RAIL_TOP, RAIL_W};
use crate::app::state::VideoApp;
use crate::catalog::folders::{in_folder, LABELS};
use crate::catalog::says::library_unavailable;
use crate::ui::format::count;
use crate::ui::icon;
use crate::ui::target::Target;
use crate::ui::text::{center_y, BODY_PX};

/// The rail: each row a `NavItem` body, the folder on show active, with the
/// player's folder glyph, the folder's name and how many videos it holds.
fn paint_rail(fb: &mut PaintBuffer, app: &VideoApp, body: Rect) {
    let theme = roles();
    let head = Rect { x: body.x, y: body.y, w: RAIL_W, h: RAIL_TOP };
    Label::new("FOLDERS").ink(Ink::Muted).draw(fb, head);
    let items = &app.browse.items;
    for i in 0..RAIL_ROWS {
        let r = rail_row(body, i);
        let (name, n) = match i {
            0 => ("All folders", items.len()),
            _ => (LABELS[i - 1], items.iter().filter(|m| in_folder(&m.path, i - 1)).count()),
        };
        let active = app.browse.folder == i.checked_sub(1);
        NavItem::new("").active(active).state(app.look(Target::Folder(i))).draw(fb, r);
        let ink = if active { theme.accent_text } else { theme.muted };
        icon::nav::files(fb, r.x + 8, r.y + r.h.saturating_sub(18) / 2, 18, ink.argb());
        let (y, tally) = (center_y(r.y, r.h), alloc::format!("{}", n));
        text::left(fb, r.x + 34, y, name, ink, BODY_PX);
        text::right(fb, (r.x + r.w).saturating_sub(10), y, &tally, theme.muted, BODY_PX);
    }
    let rule = Rect { x: body.x + RAIL_W, y: body.y, w: 1, h: body.h };
    draw::fill(fb, rule, theme.border);
}

pub fn paint(fb: &mut PaintBuffer, app: &VideoApp, body: Rect) {
    paint_rail(fb, app, body);
    let right = folder_area(body);
    if app.browse.items.is_empty() {
        let (head, note) = library_unavailable(app.browse.scanned, app.browse.scan_error)
            .unwrap_or(("No videos found", "None of these folders holds a Motion-JPEG .avi"));
        paint_empty(fb, right, icon::nav::files, head, note);
        return;
    }
    if app.browse.is_empty() {
        let note = if app.browse.query.is_empty() {
            "No video in this folder"
        } else {
            "No video in this folder matches that search"
        };
        paint_empty(fb, right, icon::nav::files, "Nothing here", note);
        return;
    }
    if app.browse.grid {
        paint_grid(fb, app, right);
    } else {
        paint_list(fb, app, right);
    }
    let tally = count(app.browse.len(), "video", "videos");
    let b = open_button(body);
    text::left(fb, right.x, center_y(b.y, b.h), &tally, roles().muted, BODY_PX);
    Button::new("Open").kind(Kind::Primary).state(app.look(Target::Open)).draw(fb, b);
}
