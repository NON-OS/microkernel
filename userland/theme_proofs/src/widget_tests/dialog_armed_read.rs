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

//! Dialog: an armed dialog's Danger label reads as body text at 4.5:1 on
//! the `danger` fill plain, hovered, pressed, held, focused and all at once,
//! in every scheme and high contrast, read back from the pixels drawn. Most
//! of the armed button's rect is `danger`, and under half of the unarmed.

use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Dialog, State};

use super::boxes::boxes;
use super::dialog::{BODIES, THREE};
use super::dialog_armed_paint::painted;
use super::dialog_row::drawn;
use super::hide::Hide;
use super::read::{reach, Off, Read};
use super::serial::{per_theme, serial};
use crate::wcag::BODY;

/// The Danger button's label in `THREE`.
const LABEL: &str = "Löschen";

/// The armed Danger button lit every way, read with its label hidden.
fn reads() -> Vec<Read<Dialog<'static>>> {
    let (d, on) = (Dialog::new("Löschen?").body(BODIES[1]).buttons(THREE).armed(true), Some(2));
    let held = d.hovered(on).pressed(on);
    let lit = [("plain", d), ("hovered", d.hovered(on)), ("pressed", d.pressed(on))];
    let lit = [lit, [("held", held), ("focused", d.focus(2)), ("all", held.focus(2))]].concat();
    let read = |(what, on)| Read { what, on, off: Off::Hide(Hide::Text(LABEL)), floor: BODY };
    lit.into_iter().map(read).collect()
}

/// How many pixels of the Danger button in `THREE` are `danger`, and its area.
fn red(armed: bool) -> (usize, usize) {
    let d = Dialog::new("Löschen?").body(BODIES[1]).buttons(THREE).armed(armed);
    let at = boxes(d.size())[0];
    let Some(r) = drawn(d, 2, at) else { return (0, 0) };
    let c = &painted(d, at);
    let px = (r.y..r.y + r.h).flat_map(|y| (r.x..r.x + r.w).map(move |x| c.get(x, y)));
    (px.filter(|&p| p == roles().danger.argb()).count(), (r.w * r.h) as usize)
}

#[test]
fn dialog_armed_danger_label_reads_on_its_fill() {
    let _g = serial();
    let mut fails = Vec::new();
    per_theme(|theme| {
        for read in reads() {
            let r = reach(&read, State::default(), roles().bg.argb());
            if r < read.floor {
                fails.push(format!("{theme}: armed {} {r:.2}:1", read.what));
            }
        }
        let ((on, area), (off, _)) = (red(true), red(false));
        if on * 2 <= area || off * 2 > area {
            fails.push(format!("{theme}: {on} armed, {off} unarmed of {area} are danger"));
        }
    });
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
