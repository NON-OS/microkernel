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

//! Dialog: asked for more lines than its box holds, the body is cut and
//! `cut` says so, and the title, the body's lines and the note stay apart,
//! inside the panel and above the buttons, with and without the note.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::{Dialog, Tone};

use super::boxes::boxed;
use super::dialog::{NOTE, SETS};
use super::dialog_armed_place::unlit;
use super::dialog_lines::list;
use super::dialog_row::drawn;
use super::dialog_spans::spans;
use super::serial::{serial, SCALES};

#[test]
fn dialog_asked_for_lines_in_a_short_box_is_cut_and_keeps_apart() {
    let _g = serial();
    let list = list();
    for q in SCALES {
        set_scale(q);
        let d = Dialog::new("Install package?").body(&list).buttons(SETS[2]).body_lines(64);
        for d in [d, d.note(NOTE, Tone::Warn)] {
            let (w, h) = d.size();
            for at in [boxed(w, h * 2 / 3), boxed(w, h / 2), boxed(w, h / 3)] {
                let (s, p) = (spans(d, at), d.panel(at));
                let floor = (0..2).filter_map(|i| drawn(unlit(d), i, at)).map(|r| r.y as i32);
                let floor = floor.min().unwrap_or(-1);
                let text = &s[..s.len() - 2];
                let apart = text.windows(2).all(|t| t[0].bottom <= t[1].top);
                let low = text.iter().all(|t| t.top >= p.y as i32 && t.bottom <= floor);
                assert!(d.cut(at) && apart && low, "{q}/4 in {at:?} over {floor}: {text:?}");
            }
        }
    }
    set_scale(SCALES[0]);
}
