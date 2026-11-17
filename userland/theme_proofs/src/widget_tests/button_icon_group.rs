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

//! Button with an icon and a label: the two are centred in the body as one
//! group, both in the label's ink, and the icon's side and gap widen `size`.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::{ICON as SIDE, SPACE};
use nonos_toolkit::design::text_role::TextRole;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::rect::centre;
use nonos_toolkit::widgets::{draw, text, Button, Kind};

use super::button_icon::{paint, AT};
use super::icon_button::ICON;
use super::serial::serial;

#[test]
fn button_icon_and_label_are_centred_as_one_group() {
    let _g = serial();
    let (s, r) = (scale(), roles());
    let (side, gap, px) = (s.px(SIDE), s.px(u32::from(SPACE.sm)), s.font(TextRole::Body.px()));
    let group = side + gap + text::width("Save", px);
    let x = AT.x + (AT.w - group) / 2;
    let got = paint(|fb| Button::new("Save").icon(ICON).kind(Kind::Primary).draw(fb, AT));
    let want = paint(|fb| {
        Button::new("").kind(Kind::Primary).draw(fb, AT);
        let top = text::centre_top(AT.y, AT.h, px);
        text::left(fb, x + side + gap, top, "Save", r.on_accent, px);
        draw::icon(fb, ICON, x, centre(AT, side, side).y, side, r.on_accent);
    });
    assert!(got == want, "icon and label are not one centred group in label ink");
    let lone = Button::new("Save").size().0;
    assert_eq!(Button::new("Save").icon(ICON).size().0, lone + side + gap);
}
