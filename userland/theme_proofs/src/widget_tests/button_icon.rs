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

//! Button with an icon: alone it is centred in the body in the label's ink,
//! disabled included, checked against the body drawn bare with the icon laid
//! where it belongs.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::ICON as SIDE;
use nonos_toolkit::theme::{roles, Opaque};
use nonos_toolkit::widgets::rect::centre;
use nonos_toolkit::widgets::{draw, Button, Kind, Rect, State};

use super::canvas::Canvas;
use super::icon_button::ICON;
use super::serial::serial;

/// The box every icon proof draws in.
pub(crate) const AT: Rect = Rect { x: 24, y: 24, w: 200, h: 36 };

/// The canvas after `f` has painted round `AT`.
pub(crate) fn paint(f: impl FnOnce(&mut nonos_toolkit::paint::PaintBuffer<'_>)) -> Vec<u32> {
    Canvas::new(AT, 0xFF12_3456).painted(f).px
}

fn bare_icon(kind: Kind, st: State, ink: Opaque) -> (Vec<u32>, Vec<u32>) {
    let side = scale().px(SIDE);
    let got = paint(|fb| Button::new("").icon(ICON).kind(kind).state(st).draw(fb, AT));
    let want = paint(|fb| {
        Button::new("").kind(kind).state(st).draw(fb, AT);
        let i = centre(AT, side, side);
        draw::icon(fb, ICON, i.x, i.y, side, ink);
    });
    (got, want)
}

#[test]
fn button_icon_alone_is_centred_in_the_body() {
    let _g = serial();
    let (got, want) = bare_icon(Kind::Secondary, State::default(), roles().text);
    assert!(got == want, "a bare icon is not the centred icon in label ink");
}

#[test]
fn button_disabled_icon_takes_the_disabled_ink() {
    let _g = serial();
    let off = State { disabled: true, ..State::default() };
    for kind in [Kind::Primary, Kind::Secondary, Kind::Plain, Kind::Danger] {
        let (got, want) = bare_icon(kind, off, roles().disabled);
        assert!(got == want, "{kind:?} disabled icon is not in disabled ink");
    }
}
