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

//! Tabs: the selected label (`accent_text`) and the other labels (`muted`),
//! hovered or not, read on the strip in every state, as do the icons in
//! their label's ink and the busy dot in `accent_text`, as marks at 3:1.

use nonos_toolkit::widgets::Tabs;

use super::hide::Hide;
use super::legible::{assert_legible, Off, Read};
use super::serial::serial;
use super::tabs::{ICONS, LABELS};
use crate::wcag::{BODY, LARGE};

#[test]
fn tab_labels_read_selected_hovered_or_plain() {
    let _g = serial();
    let hide = Off::Hide(Hide::Text(LABELS[1]));
    let plain = Tabs::new(&LABELS, 0);
    assert_legible(&[
        Read { what: "selected", on: Tabs::new(&LABELS, 1), off: hide, floor: BODY },
        Read { what: "unselected", on: plain, off: Off::Hide(Hide::Text(LABELS[1])), floor: BODY },
        Read {
            what: "hovered",
            on: plain.hovered(Some(1)),
            off: Off::Hide(Hide::Text(LABELS[1])),
            floor: BODY,
        },
    ]);
}

#[test]
fn tab_icon_and_busy_dot_read_selected_hovered_or_plain() {
    let _g = serial();
    let icon = |at| Tabs::new(&LABELS, at).icons(&ICONS[..1]);
    let dot = |at| Tabs::new(&LABELS, at).busy(&[true]);
    let (icons, dots) = (|| Off::Hide(Hide::Icons), || Off::Hide(Hide::Dots));
    assert_legible(&[
        Read { what: "selected icon", on: icon(0), off: icons(), floor: LARGE },
        Read { what: "icon", on: icon(9), off: icons(), floor: LARGE },
        Read { what: "hovered icon", on: icon(9).hovered(Some(0)), off: icons(), floor: LARGE },
        Read { what: "selected dot", on: dot(0), off: dots(), floor: LARGE },
        Read { what: "dot", on: dot(9), off: dots(), floor: LARGE },
        Read { what: "hovered dot", on: dot(9).hovered(Some(0)), off: dots(), floor: LARGE },
    ]);
}
