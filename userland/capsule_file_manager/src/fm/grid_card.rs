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

use alloc::string::{String, ToString};

use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Card, Rect, State as Look, Surface};

use super::entries::Entry;
use super::file_color::color;
use super::file_kind::kind_of;
use super::human_size::human_size;
use super::icon;
use super::layout::{font, px};
use super::list_kind_label::kind_label;

const GLYPH: u32 = 44;
const GLYPH_DY: u32 = 10;
const NAME_PX: f32 = 14.0;
const META_PX: f32 = 13.0;

/// One grid card on the shared `Card`: a large typed glyph, the measured name,
/// and a meta line. This capsule has no image or PDF decoder, so a card shows
/// its filetype glyph rather than faking a preview. A checked card draws
/// selected and the cursor's card focused.
pub fn card(fb: &mut impl Surface, entry: &Entry, at: Rect, lit: Look) {
    let (r, Rect { x, y, w, .. }) = (roles(), at);
    Card::new().state(lit).draw(fb, at);
    let (tint, bg, glyph, name_px) =
        (color(kind_of(entry)), r.surface.argb(), px(GLYPH), font(NAME_PX));
    let (gx, gy) = (x + w.saturating_sub(glyph) / 2, y + px(GLYPH_DY));
    if entry.is_dir {
        icon::folder(fb, gx, gy, glyph, tint, bg);
    } else {
        icon::file(fb, gx, gy, glyph, tint, bg);
    }
    let name = text::truncate(entry.label.trim_end_matches('/'), w.saturating_sub(px(16)), name_px);
    let ink = if lit.selected || lit.focused { r.text } else { r.text_2 };
    text::centre(fb, at, (gy + glyph + px(4)) as i32, &name, ink, name_px);
    let meta = meta_line(entry);
    text::centre(fb, at, (gy + glyph + px(26)) as i32, &meta, r.muted, font(META_PX));
}

/// A file's size where the walk reported one, and its kind otherwise, so the
/// line under a name is never blank.
fn meta_line(entry: &Entry) -> String {
    match entry.size {
        Some(size) if !entry.is_dir => human_size(size),
        _ => kind_label(kind_of(entry)).to_string(),
    }
}
