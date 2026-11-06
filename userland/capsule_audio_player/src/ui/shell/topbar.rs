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

//! The top strip over the content column: a search field that mirrors the live
//! query buffer, and a right-hand chip naming the mount the library was
//! scanned from. Clicking the field routes to the Search screen.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, text, Separator};

use crate::ui::geometry::Rect;
use crate::ui::hover::Lit;
use crate::ui::icon::{Glyph, Icons};
use crate::ui::marks::glyph_on;
use crate::ui::metrics::{pill, ITEM, LABEL, S3, S4, S6};
use crate::ui::target::Target;

use super::chrome_geom::{clear, field, CHIP_W, FIELD_H};

/// The line top that centres one line at `px` down `r`.
fn mid(r: Rect, px: f32) -> i32 {
    text::centre_top(r.tk().y, r.tk().h, px)
}

pub fn topbar(fb: &mut PaintBuffer, icons: &Icons, r: Rect, query: &str, focused: bool, lit: &Lit) {
    let t = roles();
    let text_2 = t.text_2.argb();
    Separator::new().draw(fb, Rect::new(r.x, r.bottom() - 1, r.w, 1).tk());

    let f = field(r);
    let (body, rad) = (f.tk(), pill(f.h));
    draw::round(fb, body, rad, t.sunken);
    if focused {
        draw::round(fb, body, rad, t.accent_wash);
        draw::outline(fb, body, rad, 1, t.focus_ring);
    } else {
        if lit.of(Target::Field).hovered {
            draw::round(fb, body, rad, t.hover_wash);
        }
        draw::outline(fb, body, rad, 1, t.border);
    }
    icons.centred(fb, Rect::new(f.x + S3, f.y, 22, f.h), 16, Glyph::Search, text_2);
    let tx = f.x + S3 + 22 + S3;
    let text_w = f.right() - tx - S4 - if query.is_empty() { 0 } else { 24 };
    /* A pasted address runs far past the field: its end, the file's name,
     * is what shows. */
    let shown = text::tail(query, text_w.max(0) as u32, ITEM);
    let (label, ink) = if query.is_empty() { ("Search the library", t.quiet) } else { (shown.as_str(), t.text) };
    text::left(fb, tx.max(0) as u32, mid(f, ITEM), label, ink, ITEM);
    if !query.is_empty() {
        glyph_on(fb, icons, clear(r), (Glyph::Close, 14, text_2), lit.of(Target::Clear));
    }

    /* The folder the library reads, as a label: there is no other source to
     * pick and no notifications to show, so no menu or bell beside it. */
    let chip = Rect::new(r.right() - S6 - CHIP_W, r.cy() - FIELD_H / 2, CHIP_W, FIELD_H);
    draw::round(fb, chip.tk(), pill(chip.h), t.raised);
    draw::outline(fb, chip.tk(), pill(chip.h), 1, t.border);
    icons.centred(fb, Rect::new(chip.x + S3, chip.y, 20, chip.h), 15, Glyph::Download, t.accent.argb());
    let (right, top) = ((chip.right() - S4).max(0) as u32, mid(chip, LABEL));
    text::right(fb, right, top, "My music", t.text_2, LABEL);
}
