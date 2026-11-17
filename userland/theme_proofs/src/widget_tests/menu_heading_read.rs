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

//! Menu: a heading reads at the floor a tag or shortcut reads at, on the
//! panel and beside lit rows, in every scheme and high contrast at 1x and
//! 2x. It draws apart from a disabled item with the same label, and as one
//! in a disabled menu.

use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Item, Menu, State};

use super::boxes::boxes;
use super::canvas::Canvas;
use super::hide::Hide;
use super::legible::{assert_legible_in, Off, Read};
use super::menu_heading::GROUPED;
use super::serial::{per_theme, serial};
use crate::wcag::BODY;

const HEAD: &str = "Früchte";

fn reads() -> Vec<Read<Menu<'static>>> {
    let m = Menu::new(&GROUPED);
    let lit = [("plain", m), ("hovered", m.hovered(Some(0))), ("next lit", m.hovered(Some(1)))];
    let read = |(what, on)| Read { what, on, off: Off::Hide(Hide::Text(HEAD)), floor: BODY };
    lit.into_iter().chain([("held", m.pressed(Some(0)))]).map(read).collect()
}

#[test]
fn menu_heading_reads_on_the_panel_and_beside_lit_rows() {
    let _g = serial();
    let off = State { disabled: true, ..State::default() };
    assert_legible_in(&reads(), || [State::default(), off].into_iter());
}

#[test]
fn menu_heading_draws_apart_from_a_disabled_item() {
    let _g = serial();
    let (head, dead) = ([Item::heading(HEAD)], [Item::new(HEAD).disabled()]);
    let mut fails = Vec::new();
    per_theme(|theme| {
        for st in [State::default(), State { disabled: true, ..State::default() }] {
            let (h, d) = (Menu::new(&head).state(st), Menu::new(&dead).state(st));
            let at = boxes(h.size())[0];
            let paint = |m: Menu| Canvas::new(at, roles().bg.argb()).painted(|fb| m.draw(fb, at));
            if (paint(h).px == paint(d).px) != st.disabled {
                fails.push(format!("{theme} {st:?}"));
            }
        }
    });
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
