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

//! What must hold after every step of a fuzz run, and the host actions a
//! run interleaves with output.

use nonos_vt::limits::MAX_REPLY;
use nonos_vt::{Pos, Term};

use super::rng::Rng;

pub fn check(t: &mut Term) {
    let c = t.cursor();
    assert!(c.x < t.cols() && c.y < t.rows(), "cursor {c:?} off a {}x{}", t.cols(), t.rows());
    if t.view_offset() == 0 {
        for y in 0..t.rows() {
            assert_eq!(t.visible_line(y).len(), t.cols(), "row {y}");
        }
    }
    assert!(t.take_replies().len() <= MAX_REPLY);
    assert_eq!(t.take_dirty().len(), t.rows());
}

fn pos(t: &Term, r: &mut Rng) -> Pos {
    Pos { line: t.first_line() + r.next() % 300, col: (r.next() % 250) as usize }
}

/// Now and then resize, scroll, copy or search, as a person would while
/// output streams in.
pub fn poke(t: &mut Term, r: &mut Rng, round: usize) {
    match r.next() % 16 {
        0 => t.resize(1 + (r.next() % 200) as usize, 1 + (r.next() % 80) as usize),
        1 => t.scroll_view((r.next() % 64) as isize - 32),
        2 => {
            let (a, b) = (pos(t, r), pos(t, r));
            let _ = t.text_between(a, b, r.next().is_multiple_of(2));
            let _ = t.word_at(a);
            let _ = t.line_bounds(a);
        }
        3 if round.is_multiple_of(8) => {
            let _ = t.find("fox", t.cursor_pos(), r.next().is_multiple_of(2), false);
        }
        _ => {}
    }
}
