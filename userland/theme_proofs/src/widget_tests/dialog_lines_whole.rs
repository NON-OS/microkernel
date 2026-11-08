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

//! Dialog: asked for more lines, a long list shows every line uncut while
//! the box has room, and exactly the lines asked for: one fewer cuts the
//! last with `…`, and unasked it stops at `DIALOG_LINES`.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::design::size::DIALOG_LINES;
use nonos_toolkit::widgets::Dialog;

use super::boxes::boxed;
use super::dialog::SETS;
use super::dialog_lines::list;
use super::dialog_spans::spans;
use super::serial::{serial, SCALES};

/// The body's lines `d` draws in its own size, and whether it is cut.
fn body(d: Dialog) -> (Vec<String>, bool) {
    let at = boxed(d.size().0, d.size().1);
    let s = spans(d, at);
    (s[1..s.len() - 2].iter().map(|l| l.text.clone()).collect(), d.cut(at))
}

#[test]
fn dialog_asked_for_lines_shows_the_whole_list() {
    let _g = serial();
    let list = list();
    for q in SCALES {
        set_scale(q);
        let d = Dialog::new("Install package?").body(&list).buttons(SETS[2]);
        let (all, cut) = body(d.body_lines(64));
        let k = all.len() as u32;
        assert!(k > DIALOG_LINES && !cut && all.join(" ") == list, "{q}/4: {all:?}");
        assert_eq!(body(d.body_lines(k)), (all.clone(), false), "{q}/4: asked for {k}");
        let (less, cut) = body(d.body_lines(k - 1));
        let ends = less.last().is_some_and(|l| l.ends_with('…')) && cut;
        assert!(less.len() as u32 == k - 1 && ends, "{q}/4: {less:?}");
        assert!(body(d).1 && body(d).0.len() as u32 == DIALOG_LINES, "{q}/4: unasked");
    }
    set_scale(SCALES[0]);
}
