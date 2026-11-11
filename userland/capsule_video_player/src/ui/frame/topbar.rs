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
use nonos_toolkit::icons::IconId;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, Field, Kind, Rect, State};

use super::region::{grid_half, list_half, search, view_toggle, HALF};
use crate::app::state::VideoApp;
use crate::ui::icon;
use crate::ui::marks::glyph_on;
use crate::ui::target::Target;

const GLYPH: u32 = 16;

/// The switch: a panel with the live half raised, and each half a bare
/// button body that lights under the pointer, its glyph accent when live.
fn paint_view(fb: &mut PaintBuffer, r: Rect, grid: bool, app: &VideoApp) {
    let (theme, s) = (roles(), scale());
    let (radius, inset, glyph) = (s.px(8), s.px(2), s.px(GLYPH));
    draw::round(fb, r, radius, theme.surface);
    draw::outline(fb, r, radius, s.hair(), theme.border);
    let live = if grid { grid_half(r) } else { list_half(r) };
    let (w, h) = (live.w.saturating_sub(inset * 2), r.h.saturating_sub(inset * 2));
    let raised = Rect { x: live.x + inset, y: r.y + inset, w, h };
    draw::round(fb, raised, s.px(6), theme.raised);
    let (x, y, h) = (r.x + s.px(HALF), r.y + s.px(6), r.h.saturating_sub(s.px(12)));
    draw::fill(fb, Rect { x, y, w: s.hair(), h }, theme.border);
    let ink = |on: bool| if on { theme.accent } else { theme.muted };
    let (g, l) = (grid_half(r), list_half(r));
    let look = |half: bool| app.look(Target::View(half));
    glyph_on(fb, g, (icon::ui::grid, glyph, ink(grid)), Kind::Plain, look(true));
    glyph_on(fb, l, (icon::ui::list, glyph, ink(!grid)), Kind::Plain, look(false));
}

/// The search field, which takes the keyboard while it holds a search.
pub fn paint_search(fb: &mut PaintBuffer, app: &VideoApp, hint: &str) {
    let (w, h) = (fb.width, fb.height);
    let query = app.browse.query.as_str();
    let typing = !query.is_empty();
    let look = State { focused: typing, ..app.look(Target::Search) };
    let field = Field::new(query).icon(IconId::SettingsSearch).placeholder(hint);
    field.caret(typing.then_some(query.len())).state(look).draw(fb, search(w, h));
}

/// The grid and list switch. The list is always sorted by name and holds only
/// playable videos, so there is no sort or filter menu beside it.
pub fn paint_tools(fb: &mut PaintBuffer, app: &VideoApp) {
    paint_view(fb, view_toggle(fb.width, fb.height), app.browse.grid, app);
}
