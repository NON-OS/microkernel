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

//! The shell's prompts as the shared `Dialog`, lit by a `Pointer`: each button
//! lights and presses only over the box its click names, leaving lights
//! nothing, and Delete's confirm is `Danger`, at each scale the scale test sets.

use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Dialog, Kind, Pointer, Rect};

use crate::shell_prompt_buttons::{Buttons, APPROVE, DELETE, INSTALL};
use crate::shell_prompt_paint::{drawn_in, hit_box, holds, inside, painted, LAYER};
use crate::shell_prompt_view::lit;

const BODY: &str = "The file is removed from your desktop. There is no Trash to get it back.";

fn lights_where_clicked(tag: &str, set: &Buttons) {
    let d = Dialog::new("Delete?").body(BODY).buttons(set).focus(1);
    for (b, other) in [(0, 1), (1, 0)] {
        let (bb, ob, mut p) = (hit_box(&d, b), hit_box(&d, other), Pointer::new());
        let mid = |r: Rect| ((r.x + r.w / 2) as i32, (r.y + r.h / 2) as i32);
        let ((x, y), (ox, oy)) = (mid(bb), mid(ob));
        p.moved(x, y);
        assert_eq!(lit(&d, LAYER, &p), (Some(b), None), "{tag} {set:?}: hover {b}");
        assert!(drawn_in(d, &p, bb), "{tag} {set:?}: hover {b} off its box");
        p.pressed(x, y);
        assert_eq!(lit(&d, LAYER, &p), (Some(b), Some(b)), "{tag} {set:?}: press {b}");
        assert!(drawn_in(d, &p, bb), "{tag} {set:?}: press {b} off its box");
        p.moved(ox, oy);
        assert_eq!(lit(&d, LAYER, &p), (Some(other), None), "{tag} {set:?}: dragged off {b}");
        p.moved(d.panel(LAYER).x as i32 - 4, y);
        assert_eq!(lit(&d, LAYER, &p), (None, None), "{tag} {set:?}: left the panel");
        assert_eq!(painted(d, &p), painted(d, &Pointer::new()), "{tag} {set:?}: left lit");
    }
}

pub(crate) fn prompts_light_where_they_are_clicked(tag: &str) {
    for set in [&APPROVE, &INSTALL, &DELETE] {
        lights_where_clicked(tag, set);
    }
    assert_eq!(DELETE.map(|(_, k)| k), [Kind::Danger, Kind::Secondary], "{tag}: Delete kinds");
    let d = Dialog::new("Delete?").body(BODY).buttons(&DELETE);
    let (danger, delete) = (roles().danger.argb(), hit_box(&d, 0));
    let words = painted(d, &Pointer::new());
    let mut reds = inside(LAYER).zip(&words).filter(|(_, &c)| c == danger).peekable();
    let on_delete = reds.peek().is_some() && reds.all(|(at, _)| holds(delete, at));
    assert!(on_delete, "{tag}: Delete is not the Danger button");
}
