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

//! Toast: given `text_lines(n)` the text takes as many lines as it needs, up
//! to n, the last cut with `…` only when more would follow; a notice that
//! needs n lines or fewer is shown whole in its own box and wider ones, at
//! the width asked for with `max_width`, scaled, where two would cut it.

use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::widgets::Toast;

use super::boxes::boxes;
use super::serial::{serial, SCALES};
use super::toast::OVER;
use super::toast_lines::Runs;

/// A notice as long as the Browser's warning for a short name, which must be
/// read whole across a page.
pub(crate) const NOTICE: &str = "Reached by a short .anyone name from the list the Anyone \
    DNS services sign. A short name is weaker than the full address: the list decides where \
    it points. NONOS refuses a name that changes service within a boot.";

/// The lines `t` draws in its `i`th box.
fn drawn(t: Toast, i: usize) -> Vec<String> {
    let mut runs = Runs(Vec::new());
    t.draw(&mut runs, boxes(t.size())[i]);
    runs.0
}

#[test]
fn toast_takes_up_to_the_lines_asked_and_cuts_only_past_them() {
    let _g = serial();
    let mut fails = Vec::new();
    for q in SCALES {
        set_scale(q);
        for (text, w) in [(OVER, 360), (NOTICE, 600), (NOTICE, 360)] {
            let need = drawn(Toast::new(text).max_width(w).text_lines(99), 0).len();
            for n in 1..=need as u32 + 1 {
                let t = Toast::new(text).max_width(w).text_lines(n);
                for i in 0..2 {
                    let lines = drawn(t, i);
                    let cut = lines.last().is_some_and(|l| l.ends_with('…'));
                    let k = n.min(need as u32) as usize;
                    let own = lines.len() == k && cut == (k < need);
                    if !(if i == 0 { own } else { k < need || !cut }) || lines.is_empty() {
                        fails.push(format!("{w} wide, {n} of {need} at {q}/4 box {i}: {lines:?}"));
                    }
                }
            }
        }
        let (cap, notice) = (scale().px(600), Toast::new(NOTICE).max_width(600).text_lines(3));
        let words = |t: &str| t.split_whitespace().map(String::from).collect::<Vec<_>>();
        let (lines, two) = (drawn(notice, 0), drawn(Toast::new(NOTICE).max_width(600), 0));
        let whole = lines.len() == 3 && words(&lines.join(" ")) == words(NOTICE);
        if !whole || notice.size().0 != cap || !two[1].ends_with('…') {
            fails.push(format!("at {q}/4: {:?} {:?}", notice.size(), drawn(notice, 0)));
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
