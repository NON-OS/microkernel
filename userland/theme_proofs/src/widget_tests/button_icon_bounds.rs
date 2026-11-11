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

//! Button with an icon: in every kind and state, alone or with a label, at
//! every scale, nothing is drawn outside the box, down to boxes narrower and
//! shorter than the icon itself.

use nonos_toolkit::widgets::Button;

use super::bounds::assert_bounded_in;
use super::boxes::boxed;
use super::button::KINDS;
use super::icon_button::ICON;
use super::samples::LONG;
use super::serial::serial;

#[test]
fn button_icon_never_escapes_a_small_box() {
    let _g = serial();
    let mut all = Vec::new();
    for (name, kind) in KINDS {
        for label in ["", "Save", LONG] {
            all.push((format!("{name} {label:?}"), Button::new(label).icon(ICON).kind(kind)));
        }
    }
    let place = |(w, h): (u32, u32)| {
        let sizes = [(w, h), (w / 2, h), (40, h), (24, h), (12, 12), (6, 30), (1, 1), (0, h)];
        sizes.map(|(w, h)| boxed(w, h))
    };
    assert_bounded_in(&all, place);
}
