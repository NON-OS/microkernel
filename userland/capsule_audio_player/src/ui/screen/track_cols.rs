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

//! The track table's columns: the index, the cover, the title over the
//! artist, the album, the time and the queued mark; and the head naming them.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Separator};

use crate::ui::geometry::Rect;
use crate::ui::metrics::{LABEL, S3, S4};

use super::lib_geom::ROW_H;

pub struct Cols {
    pub index: Rect,
    pub art: Rect,
    pub title: Rect,
    pub format: Rect,
    pub time: Rect,
    pub pip: Rect,
}

pub fn cols(r: Rect) -> Cols {
    let art_s = ROW_H - S4;
    let ix = Rect::new(r.x + S3, r.y, 34, r.h);
    let art = Rect::new(ix.right() + S3, r.cy() - art_s / 2, art_s, art_s);
    let pip = Rect::new(r.right() - 34, r.y, 26, r.h);
    let time = Rect::new(pip.x - 80, r.y, 70, r.h);
    /* Room for an album's name; the title keeps the rest. */
    let format = Rect::new(time.x - 236, r.y, 220, r.h);
    Cols {
        index: ix,
        art,
        title: Rect::new(art.right() + S4, r.y, format.x - art.right() - S4 * 2, r.h),
        format,
        time,
        pip,
    }
}

/// The left edge of `r`, and the line top that centres one line at `px` in it.
pub fn at(r: Rect, px: f32) -> (u32, i32) {
    (r.tk().x, text::centre_top(r.tk().y, r.tk().h, px))
}

pub fn header(fb: &mut PaintBuffer, r: Rect) {
    let (c, muted) = (cols(r), roles().muted);
    for (col, name) in [(c.index, "#"), (c.title, "TITLE"), (c.format, "ALBUM")] {
        let (x, top) = at(col, LABEL);
        text::left(fb, x, top, name, muted, LABEL);
    }
    let (x, top) = at(c.time, LABEL);
    text::right(fb, x + c.time.tk().w, top, "TIME", muted, LABEL);
    Separator::new().draw(fb, Rect::new(r.x, r.bottom() - 1, r.w, 1).tk());
}
