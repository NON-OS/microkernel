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

//! What the button proofs compare a drawn button with: its inks over its bare
//! body, and the colour at the body's centre.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::{ICON as SIDE, SPACE};
use nonos_toolkit::design::text_role::TextRole;
use nonos_toolkit::theme::Opaque;
use nonos_toolkit::widgets::rect::centre;
use nonos_toolkit::widgets::{draw, text, Button, Kind, State};

use super::button_icon::{paint, AT};
use super::icon_button::ICON;
use super::serial::per_theme;

/// Whether a `kind` button in `st` draws its icon and label, centred as one
/// group, in `ink` over its bare body.
pub(crate) fn inked(kind: Kind, st: State, ink: Opaque) -> bool {
    let (s, b) = (scale(), Button::new("").kind(kind).state(st));
    let (side, gap, px) = (s.px(SIDE), s.px(u32::from(SPACE.sm)), s.font(TextRole::Body.px()));
    let x = AT.x + (AT.w - side - gap - text::width("Save", px)) / 2;
    let got = paint(|fb| Button::new("Save").icon(ICON).kind(kind).state(st).draw(fb, AT));
    let want = paint(|fb| {
        b.draw(fb, AT);
        let top = text::centre_top(AT.y, AT.h, px);
        text::left(fb, x + side + gap, top, "Save", ink, px);
        draw::icon(fb, ICON, x, centre(AT, side, side).y, side, ink);
    });
    got == want
}

/// The centre of a bare `kind` body in `st`.
pub(crate) fn body(kind: Kind, st: State) -> u32 {
    let px = paint(|fb| Button::new("").kind(kind).state(st).draw(fb, AT));
    px[((AT.y + AT.h / 2) * (AT.x + AT.w + 24) + AT.x + AT.w / 2) as usize]
}

/// Runs `bad` in every theme and asserts none reports a failure.
pub(crate) fn each_theme(mut bad: impl FnMut() -> Option<String>) {
    let mut fails = Vec::new();
    per_theme(|theme| fails.extend(bad().map(|e| format!("{theme}: {e}"))));
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
