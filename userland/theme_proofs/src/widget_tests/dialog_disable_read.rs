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

//! Dialog: a button disabled alone draws its label visibly, as a disabled
//! dialog's buttons do, whatever hover, press or focus is set on it, in
//! every scheme and high contrast, read back from the pixels drawn; and
//! every other button's label reads exactly as with none disabled.

use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Dialog, State};

use super::dialog::{BODIES, THREE};
use super::hide::Hide;
use super::read::{reach, Off, Read, NONE};
use super::serial::{per_theme, serial};

/// Button `k` of `THREE` disabled, none past the last, read with button
/// `i`'s label hidden, lit plain, hovered, pressed, held, focused and all.
fn reads(k: usize, i: usize) -> Vec<Read<Dialog<'static>>> {
    let d = Dialog::new("Löschen?").body(BODIES[1]).buttons(THREE).disable(k);
    let on = Some(i);
    let held = d.hovered(on).pressed(on);
    let lit = [("plain", d), ("hovered", d.hovered(on)), ("pressed", d.pressed(on))];
    let lit = [lit, [("held", held), ("focused", d.focus(i)), ("all", held.focus(i))]].concat();
    let read = |(what, on)| Read { what, on, off: Off::Hide(Hide::Text(THREE[i].0)), floor: NONE };
    lit.into_iter().map(read).collect()
}

#[test]
fn dialog_disabled_button_label_is_drawn_and_the_rest_read_as_before() {
    let _g = serial();
    let mut fails = Vec::new();
    per_theme(|theme| {
        for (k, i) in (0..THREE.len()).flat_map(|k| (0..THREE.len()).map(move |i| (k, i))) {
            let at = |read: &Read<Dialog>| reach(read, State::default(), roles().bg.argb());
            for (read, all) in reads(k, i).into_iter().zip(reads(usize::MAX, i)) {
                let r = at(&read);
                let ok = if i == k { r > read.floor } else { r == at(&all) };
                if !ok {
                    fails.push(format!("{theme}: {k} off, {} {i} {r:.2}:1", read.what));
                }
            }
        }
    });
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
