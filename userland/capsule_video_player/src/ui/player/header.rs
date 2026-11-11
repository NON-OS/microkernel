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
use nonos_toolkit::widgets::{draw, text, Kind, Rect};

use crate::ui::hover::Lit;
use crate::ui::icon;
use crate::ui::layout::{header_h, pad, top, Layout};
use crate::ui::marks::glyph_on;
use crate::ui::target::Target;
use crate::ui::text::{center_y, title_px};

const ARROW: u32 = 18;
const INFO: u32 = 16;
const TITLE_GAP: u32 = 14;

/// The header: back on the left, the title, and info on the right, each
/// button a bare body that lights under the pointer.
pub fn paint_header(fb: &mut PaintBuffer, l: &Layout, title: &str, lit: &Lit) {
    let (r, w, s) = (roles(), fb.width, scale());
    draw::fill(fb, Rect { x: 0, y: top(), w, h: header_h() }, r.bg);
    let (arrow, info) = (s.px(ARROW), s.px(INFO));
    glyph_on(fb, l.back, (icon::ui::back, arrow, r.muted), Kind::Plain, lit.of(Target::Back));
    glyph_on(fb, l.info, (icon::ui::info, info, r.muted), Kind::Plain, lit.of(Target::Info));
    let title_x = pad() + arrow + s.px(TITLE_GAP);
    let room = w.saturating_sub(title_x + pad() + l.info.w + s.px(12));
    let shown = text::truncate(title, room, title_px());
    text::left(fb, title_x, center_y(top(), header_h()), &shown, r.text, title_px());
}
