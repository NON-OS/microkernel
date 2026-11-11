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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Rect, Row, Separator, State as Look};

use super::icon_draw::draw;
use super::icon_path::Icon;
use super::layout::{font, hair, px};

// Shared line metrics for every non-Browse surface, so Recents, Search and Tags
// stack identically and a section label always advances by the same amount.
const LIST_ROW_H: u32 = 48;
const LABEL_ADV: u32 = 28;
const ROW_ICON: u32 = 20;
const TEXT_GAP: u32 = 16;
const TITLE_DY: u32 = 4;
const SUB_DY: u32 = 26;

// The two em sizes a row's text lines are drawn at, and the line box between
// them. `MIN_UI_PX` clamps both draw and measure, so anything measuring against
// these gets the same advance the painter did.
pub const TITLE_PX: f32 = 18.0;
pub const SUB_PX: f32 = 14.0;
pub const LINE_BOX: u32 = SUB_DY - TITLE_DY;

/// Where `screen_row` puts its text: the shared left edge, the title's top y,
/// and the sub line's. Anything drawn under those glyphs -- a search match band
/// -- measures from here rather than restating the offsets.
pub fn row_text(x: u32, y: u32) -> (u32, u32, u32) {
    (x + px(ROW_ICON + TEXT_GAP), y + px(TITLE_DY), y + px(SUB_DY))
}

/// A row's height and a section label's advance at the display scale in force.
pub fn list_row_h() -> u32 {
    px(LIST_ROW_H)
}

pub fn label_adv() -> u32 {
    px(LABEL_ADV)
}

/// One list line: filetype icon, title, its parent path beneath, and an
/// optional right-aligned meta column placed by `measure_ttf` rather than by
/// glyph count. Returns the height consumed so a column stacks without repeated
/// arithmetic. The row is a shared `Row` body drawn in `look`.
pub fn screen_row(
    fb: &mut PaintBuffer,
    x: u32,
    y: u32,
    w: u32,
    row: (&str, &str, &str),
    dir: bool,
    tint: u32,
    look: Look,
) -> u32 {
    let (h, icon, line) = (list_row_h(), px(ROW_ICON), hair());
    Row::new("").state(look).draw(fb, Rect { x, y, w, h });
    let (title, sub, meta) = row;
    let iy = y + (h - icon) / 2;
    let kind = if dir { Icon::Folder } else { Icon::Doc };
    draw(fb, kind, x, iy, icon, tint);
    let ((tx, ty, sy), r) = (row_text(x, y), roles());
    let _ = fb.text_ttf(tx as i32, ty as i32, title, r.text.argb(), font(TITLE_PX));
    let _ = fb.text_ttf(tx as i32, sy as i32, sub, r.muted.argb(), font(SUB_PX));
    if !meta.is_empty() {
        let mw = fb.measure_ttf(meta, font(14.0)).max(0) as u32;
        let mx = (x + w).saturating_sub(mw);
        let _ = fb.text_ttf(mx as i32, (y + px(4)) as i32, meta, r.muted.argb(), font(14.0));
    }
    Separator::new().draw(fb, Rect { x, y: y + h - line, w, h: line });
    h
}

/// A section heading in the label ink, returning its own advance.
pub fn section_label(fb: &mut PaintBuffer, x: u32, y: u32, text: &str) -> u32 {
    let _ = fb.text_ttf(x as i32, y as i32, text, roles().muted.argb(), font(13.0));
    label_adv()
}

/// A centred two-line honest empty state, measured so it stays centred at any
/// window width.
pub fn empty_state(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, title: &str, note: &str) {
    let (iw, r, (title_px, note_px)) = (px(40), roles(), (font(20.0), font(15.0)));
    let (muted, quiet) = (r.muted.argb(), r.quiet.argb());
    draw(fb, Icon::Folder, x + w.saturating_sub(iw) / 2, y, iw, quiet);
    let tw = fb.measure_ttf(title, title_px).max(0) as u32;
    let tx = (x + w.saturating_sub(tw) / 2) as i32;
    let _ = fb.text_ttf(tx, (y + px(60)) as i32, title, muted, title_px);
    let nw = fb.measure_ttf(note, note_px).max(0) as u32;
    let nx = (x + w.saturating_sub(nw) / 2) as i32;
    let _ = fb.text_ttf(nx, (y + px(92)) as i32, note, quiet, note_px);
}
