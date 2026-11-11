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

//! The player's own button looks on the shared Button, which has no builder
//! for them: its glyph before the label, and the accent glow behind the
//! button a page leads with.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::CONTROL_RADIUS;
use nonos_toolkit::design::text_role::TextRole;
use nonos_toolkit::theme::derive::with_alpha;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Button, Kind, State};

use super::geometry::Rect;
use super::icon::{Glyph, Icons};
use super::metrics::metrics;

/// `label` on a `kind` button in `r`, after `glyph` when there is one. The
/// toolkit keeps a button's label ink to itself, so the run behind a glyph
/// inks as the two kinds the player uses would.
pub fn labelled(
    fb: &mut PaintBuffer,
    icons: &Icons,
    r: Rect,
    label: &str,
    glyph: Option<Glyph>,
    kind: Kind,
    look: State,
) {
    let Some(g) = glyph else { return Button::new(label).kind(kind).state(look).draw(fb, r.tk()) };
    Button::new("").kind(kind).state(look).draw(fb, r.tk());
    let (t, px, m) = (roles(), scale().font(TextRole::Body.px()), metrics());
    let (side, s3) = (m.item as i32, m.s3);
    let ink = if kind == Kind::Primary { t.on_accent } else { t.text };
    let x = r.x + (r.w - side - s3 - text::width(label, px) as i32) / 2;
    icons.draw(fb, Rect::new(x, r.cy() - side / 2, side, side), g, ink.argb());
    let top = text::centre_top(r.tk().y, r.tk().h, px);
    text::left(fb, (x + side + s3).max(0) as u32, top, label, ink, px);
}

/// The accent glow the player draws behind the button a page leads with.
pub fn glow(fb: &mut PaintBuffer, r: Rect) {
    let (b, rad) = (r.tk(), scale().px(CONTROL_RADIUS));
    let c = with_alpha(roles().accent.argb(), 0x4C);
    fb.shadow_round(b.x, b.y, b.w, b.h, rad, b.h / 3, c);
}
