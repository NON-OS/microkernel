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

use super::field_tests::{caret_x, draw, typing, TEXT};
use super::kit::{at_scale, changed};

/* The pixels a selection from `a` to `b` adds to the field, with the caret
 * at `b` in both. */
fn wash(ui_focus: bool, a: usize, b: usize) -> Vec<(i32, i32)> {
    let (mut sel, mut bare) = (typing(a, b), typing(b, b));
    if !ui_focus {
        (sel.focus, bare.focus) = (None, None);
    }
    changed(&draw(&bare, true), &draw(&sel, true))
}

#[test]
fn a_selection_is_drawn_only_over_the_selected_text() {
    let mid = TEXT.find('G').unwrap();
    for q in [4, 8] {
        let _g = at_scale(q);
        for (a, b) in [(0, mid), (TEXT.len(), mid), (mid, TEXT.len())] {
            let (l, r) = (caret_x(a.min(b)), caret_x(a.max(b)));
            let px = wash(true, a, b);
            assert!(!px.is_empty(), "{a}..{b} at {q} draws nothing");
            assert!(px.iter().all(|&(x, _)| x >= l && x < r), "{a}..{b} at {q}");
            assert!(wash(false, a, b).is_empty(), "{a}..{b} unfocused at {q}");
        }
    }
}
