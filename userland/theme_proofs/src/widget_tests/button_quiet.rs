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

//! Button, quiet kind: label and icon take `muted`, disabled takes over, and
//! hover and press still move the body it does not otherwise draw.

use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Kind, State};

use super::button_drawn::{body, each_theme, inked};
use super::serial::serial;

#[test]
fn button_quiet_inks_label_and_icon_muted() {
    let _g = serial();
    let off = State { disabled: true, ..State::default() };
    each_theme(|| {
        let r = roles();
        let rest = inked(Kind::Quiet, State::default(), r.muted);
        let dead = inked(Kind::Quiet, off, r.disabled);
        (!rest || !dead).then(|| format!("muted {rest} disabled {dead}"))
    });
}

#[test]
fn button_quiet_hover_and_press_move_the_body() {
    let _g = serial();
    let rest = State::default();
    each_theme(|| {
        let at = |s| body(Kind::Quiet, s);
        let (a, h) = (at(rest), at(State { hovered: true, ..rest }));
        let p = at(State { pressed: true, ..rest });
        let washes = roles().hover_wash.argb() >> 24 != 0;
        (a == h && washes || a == p).then(|| format!("rest {a:08x} hover {h:08x} press {p:08x}"))
    });
}
