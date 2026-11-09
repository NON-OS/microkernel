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
use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::{CONTROL_H, ICON, SPACE};
use nonos_toolkit::icons::IconId;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, Controls, Ink, Row};

use crate::browser::paint::pages::cells::{px, tile, WIDE_TILE};
use crate::browser::paint::pages::rows::{self, place};
use crate::browser::paint::start::tile_color;
use crate::browser::shell::downloads::human_bytes;
use crate::browser::shell::listing::download_rows;
use crate::browser::shell::types::DlState;
use crate::browser::shell::visits::host_of;
use crate::browser::shell::Shell;
use crate::browser::ui::list_geom::ListGeom;
use crate::browser::ui::page_key::key;
use crate::browser::ui::page_part::PagePart;

const SAVING: &str = "Saving\u{2026}";
const SAVED: &str = "Saved to /home/Downloads";
const NONE_YET: &str = "No downloads yet. Files the page cannot show are saved here.";
const NONE_HERE: &str = "No downloads match this filter.";

/// The Downloads list, newest first, under the state filter in force: each
/// row's extension tile, name over host and size, where it went or why
/// not, and the button that drops the record.
pub fn paint(shell: &Shell, g: &ListGeom, fb: &mut PaintBuffer, c: &mut Controls<PagePart>) {
    let list = download_rows(shell);
    if list.is_empty() {
        let msg = if shell.downloads.is_empty() { NONE_YET } else { NONE_HERE };
        return rows::empty(fb, g, msg);
    }
    rows::frame(fb, g, list.len());
    let gap = px(u32::from(SPACE.sm));
    for (i, &k) in list.iter().enumerate() {
        let (r, d) = (g.row_rect(i), &shell.downloads[k]);
        if !rows::shown(fb, g, r) {
            continue;
        }
        let (said, ink, mark) = match &d.state {
            DlState::Saving => (SAVING, Ink::Accent, None),
            DlState::Saved => (SAVED, Ink::Muted, Some((IconId::Check, Ink::Ok))),
            DlState::Failed(why) => (why.as_str(), Ink::Muted, Some((IconId::Close, Ink::Danger))),
        };
        let below = format!("{} \u{b7} {}", host_of(&d.url), human_bytes(d.bytes));
        let row = Row::new(&d.name).note(&below).value(said).value_ink(ink);
        let row =
            row.leading(px(WIDE_TILE) as u32).trailing((px(ICON) + gap + px(CONTROL_H)) as u32);
        let rm = PagePart::Remove(k, key(&d.url, d.at_ms));
        let (lead, trail) = place(fb, g, c, (r, row), (None, Some(rm)));
        let ext = ext_of(&d.name);
        tile(fb, lead, &ext, tile_color(&ext), px(WIDE_TILE));
        if let Some((id, ink)) = mark {
            let side = scale().px(ICON).min(trail.h);
            let y = trail.y + (trail.h - side) / 2;
            draw::icon(fb, id, trail.x, y, side, ink.colour(&roles()));
        }
    }
}

/* What the tile of a file named `name` says: its extension in capitals,
 * at most four letters, or `FILE` when it has none. */
fn ext_of(name: &str) -> String {
    match name.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() && !ext.is_empty() => {
            ext.chars().take(4).collect::<String>().to_ascii_uppercase()
        }
        _ => String::from("FILE"),
    }
}
