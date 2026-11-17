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
use nonos_toolkit::widgets::{draw, text, NavItem, Rect, State};

use super::geom::{brand, collapsed, nav_item, sidebar};
use super::storage::paint_storage;
use crate::app::state::VideoApp;
use crate::ui::icon::nav;
use crate::ui::marks::Glyph;
use crate::ui::screen::{Route, NAV};
use crate::ui::target::Target;
use crate::ui::text::{body_px, title_px};

const ICON: u32 = 20;
const ICON_X: u32 = 11;
const LABEL_X: u32 = 42;

pub fn paint_sidebar(fb: &mut PaintBuffer, app: &VideoApp) {
    let (w, h) = (fb.width, fb.height);
    let (s, r) = (sidebar(w, h), roles());
    draw::fill(fb, s, r.sunken);
    let hair = scale().hair();
    draw::fill(fb, Rect { x: s.x + s.w.saturating_sub(hair), w: hair, ..s }, r.border);
    paint_brand(fb, w);
    for (index, item) in NAV.iter().enumerate() {
        paint_item(fb, w, index, (*item, app.route()), app.look(Target::Nav(index)));
    }
    paint_storage(fb, w, h, app.browse.items.len());
}

fn paint_brand(fb: &mut PaintBuffer, w: u32) {
    let (b, r, s) = (brand(w), roles(), scale());
    let (mark, inset) = (s.px(30), s.px(6));
    draw::round(fb, Rect { w: mark, h: mark, ..b }, s.px(8), r.raised);
    nav::video(fb, b.x + inset, b.y + inset, s.px(18), r.accent.argb());
    if !collapsed(w) {
        let (x, y) = (b.x + s.px(LABEL_X), (b.y + s.px(4)) as i32);
        text::left(fb, x, y, "Video", r.text, title_px());
    }
}

/// One page of the sidebar on a `NavItem` body, which draws the page on show
/// and the hover and press of the rest; the glyph and label are the player's,
/// in the ink the body gives them.
fn paint_item(fb: &mut PaintBuffer, w: u32, index: usize, (item, on): (Route, Route), look: State) {
    let (r, active, theme) = (nav_item(w, index), item == on, roles());
    NavItem::new("").active(active).state(look).draw(fb, r);
    let (ink, s) = (if active { theme.accent_text } else { theme.muted }, scale());
    let icon = s.px(ICON);
    glyph_for(item)(fb, r.x + s.px(ICON_X), r.y + (r.h.saturating_sub(icon)) / 2, icon, ink.argb());
    if collapsed(w) {
        return;
    }
    let (x, y) = (r.x + s.px(LABEL_X), (r.y + s.px(10)) as i32);
    text::left(fb, x, y, item.label(), ink, body_px());
}

fn glyph_for(item: Route) -> Glyph {
    match item {
        Route::Files => nav::files,
        _ => nav::library,
    }
}
