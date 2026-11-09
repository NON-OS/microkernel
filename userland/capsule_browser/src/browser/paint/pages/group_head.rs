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

use alloc::format;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::design::text_role::TextRole;
use nonos_toolkit::font::ttf::line_height;
use nonos_toolkit::widgets::{text, Button, Controls, Field, Kind, SectionHeader, State};

use crate::browser::paint::pages::cells::{area, px};
use crate::browser::shell::kept::Group;
use crate::browser::shell::{groups, Page, Shell};
use crate::browser::ui::list_geom::ListGeom;
use crate::browser::ui::page_part::PagePart;
use crate::browser::ui::rect::Rect;

pub const TITLE_PX: f32 = 34.0;
const FIELD_MIN: u32 = 320;
const DELETE: &str = "Delete group";

/// The group the Tabs page is filtered to, if it is.
pub fn filtered(shell: &Shell) -> Option<&Group> {
    let id = shell.group_filter.filter(|_| shell.page == Page::Tabs)?;
    groups::find(&shell.groups, id)
}

/// The group's name on the header's title line, which takes a click to
/// rename; while renaming, the Title field around the name being typed,
/// its text where the name stood.
pub fn name_rect(g: &ListGeom, shell: &Shell, gr: &Group) -> Rect {
    let font = scale().font(TextRole::Title.px());
    let (lh, h) = (line_height(font), g.head.y);
    let Some(t) = shell.rename.as_deref() else {
        return Rect::new(g.head.x, h, text::width(&gr.name, font) as i32, lh);
    };
    let (pad, fh) = (px(u32::from(SPACE.md)), field(t, State::default()).size().1 as i32);
    let w = (text::width(t, font) as i32 + 3 * pad).max(px(FIELD_MIN));
    Rect::new(g.head.x - pad, h + (lh - fh) / 2, w, fh)
}

/// The Delete group button, at the right end of the pill row.
pub fn delete_rect(g: &ListGeom) -> Rect {
    let (w, h) = Button::new(DELETE).kind(Kind::Danger).size();
    let (w, h) = (w as i32, h as i32);
    Rect::new(g.frame.right() - w, g.pills_y + (g.pill_h - h) / 2, w, h)
}

/// The header of the Tabs page filtered to `gr`: its name, or the focused
/// rename field over where it stood, a line saying how many tabs it holds,
/// and Delete group.
pub fn paint(
    shell: &Shell,
    gr: &Group,
    tabs: usize,
    g: &ListGeom,
    fb: &mut PaintBuffer,
    c: &mut Controls<PagePart>,
) {
    let sub = format!("{} open in this group. Click the name to rename it", tabs);
    let renaming = shell.rename.as_deref();
    SectionHeader::new(renaming.map_or(gr.name.as_str(), |_| "")).note(&sub).draw(fb, area(g.head));
    let n = area(name_rect(g, shell, gr));
    let st = c.place(n, PagePart::Name);
    if let Some(t) = renaming {
        field(t, State { focused: true, ..st }).draw(fb, n);
    }
    let d = area(delete_rect(g));
    let st = c.place(d, PagePart::Delete);
    Button::new(DELETE).kind(Kind::Danger).state(st).draw(fb, d);
}

/* The rename field holding `t`, the caret at its end. */
fn field(t: &str, st: State) -> Field<'_> {
    Field::new(t).role(TextRole::Title).caret(Some(t.len())).state(st)
}
