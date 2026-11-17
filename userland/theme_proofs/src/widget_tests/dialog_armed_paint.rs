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

//! Dialog: arming changes only the pixels inside its Danger buttons, and
//! some when one is drawn; `armed(false)` changes no pixel, and a disabled
//! dialog draws the same armed or not. `dialogs` lays the sets out in
//! `SETS` order under each body, so the set of each is `SETS` cycled.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Dialog, Kind, Rect, State};

use super::boxes::{boxes, inside};
use super::canvas::Canvas;
use super::dialog::{dialogs, Buttons, SETS};
use super::dialog_armed_place::unlit;
use super::dialog_row::drawn;
use super::serial::{serial, SCALES};

/// `d` drawn in `at` over the scheme's `bg`.
pub(crate) fn painted(d: Dialog, at: Rect) -> Canvas {
    Canvas::new(at, roles().bg.argb()).painted(|fb| d.draw(fb, at))
}

/// A line if arming `d` changes a pixel outside its Danger buttons, or
/// none though one is drawn, or if it changes the disabled dialog, or if
/// `armed(false)` changes anything.
fn strays(d: Dialog, set: Buttons, at: Rect) -> Option<String> {
    let danger = (0..set.len()).filter(|&i| set[i].1 == Kind::Danger);
    let danger: Vec<Rect> = danger.filter_map(|i| drawn(unlit(d), i, at)).collect();
    let plain = painted(d, at);
    let diff = plain.diff(&painted(d.armed(true), at));
    let off = diff.iter().find(|&&pt| !danger.iter().any(|&r| inside(r, pt)));
    let dead = d.state(State { disabled: true, ..State::default() });
    let dead = painted(dead, at).diff(&painted(dead.armed(true), at)).len();
    let same = plain.diff(&painted(d.armed(false), at)).len();
    let bad = off.is_some() || dead + same > 0 || diff.is_empty() != danger.is_empty();
    bad.then(|| format!("{off:?} of {} off {danger:?}; {dead} dead, {same} unarmed", diff.len()))
}

#[test]
fn dialog_arming_fills_only_its_danger_buttons() {
    let _g = serial();
    let mut fails = Vec::new();
    for q in SCALES {
        set_scale(q);
        let sets = dialogs().into_iter().zip(SETS.into_iter().cycle());
        for ((name, d), set) in sets {
            for at in boxes(d.size()) {
                fails.extend(strays(d, set, at).map(|e| format!("{name} at {q}/4 in {at:?}: {e}")));
            }
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
