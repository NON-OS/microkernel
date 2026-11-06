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

//! Dialog: `cut` says the body leaves some of itself out exactly when it
//! does: a body that fits its own box is whole, the over-long one and a
//! word wider than the panel are cut, and adding the note uncuts nothing.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::{Dialog, Tone};

use super::boxes::boxed;
use super::dialog::{BODIES, NOTE, OVER};
use super::serial::{serial, SCALES};

fn cut_at_own_size(body: &str, note: bool) -> bool {
    let d = Dialog::new("Delete?").body(body);
    let d = if note { d.note(NOTE, Tone::Warn) } else { d };
    let (w, h) = d.size();
    d.cut(boxed(w, h))
}

#[test]
fn dialog_cut_is_the_body_left_out() {
    let _g = serial();
    let wide = "W".repeat(400);
    for q in SCALES {
        set_scale(q);
        for body in ["", BODIES[0], BODIES[1], BODIES[3]] {
            assert!(!cut_at_own_size(body, false), "{q}/4: {body:?} is cut");
        }
        for (name, body) in [("over-long", OVER), ("one wide word", wide.as_str())] {
            assert!(cut_at_own_size(body, false), "{q}/4: the {name} body is whole");
            assert!(cut_at_own_size(body, true), "{q}/4: the note uncut the {name} body");
        }
    }
}
