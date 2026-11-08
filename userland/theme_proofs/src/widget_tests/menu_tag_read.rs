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

//! Menu: a row's tag reads at 4.5:1, the floor its shortcut clears, on the
//! panel, on the hover wash and on a pressed and held row, in every scheme
//! and high contrast, read back from the pixels drawn. A disabled row's tag,
//! and every tag in a disabled menu, need only be drawn visibly.

use nonos_toolkit::widgets::{Item, Menu, State};

use super::hide::Hide;
use super::legible::{assert_legible_in, Off, Read, NONE};
use super::serial::serial;
use crate::wcag::BODY;

const TAG: &str = "run";
const OFF: &str = "dir";

/// A tagged item with keys and a tick, and a disabled tagged one.
static ITEMS: [Item; 2] = [
    Item::new("Grüße — 日本語").shortcut("Strg+Ü").checked(true).tag(TAG),
    Item::new("Löschen").tag(OFF).disabled(),
];

/// Every ink to read: the live tag plain, hovered, pressed and held, then
/// the disabled one.
fn reads() -> Vec<Read<Menu<'static>>> {
    let m = Menu::new(&ITEMS);
    let (h, p) = (m.hovered(Some(0)), m.pressed(Some(0)));
    let lit = [("plain", m), ("hovered", h), ("pressed", p), ("held", h.pressed(Some(0)))];
    let read = |what, on, run, floor| Read { what, on, off: Off::Hide(Hide::Text(run)), floor };
    let mut all: Vec<_> = lit.into_iter().map(|(what, on)| read(what, on, TAG, BODY)).collect();
    all.push(read("disabled", m, OFF, NONE));
    all
}

#[test]
fn menu_tags_read_on_the_panel_and_every_lit_row() {
    let _g = serial();
    let off = State { disabled: true, ..State::default() };
    assert_legible_in(&reads(), || [State::default(), off].into_iter());
}
