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

//! The canonical-mode line editing a foreground program reads through, and
//! which cells a pointer selection covers.

use crate::cooked::{Cooked, Effect};
use crate::select::Selection;
use nonos_vt::Pos;

fn type_str(c: &mut Cooked, s: &str, fx: &mut Effect) {
    for ch in s.chars() {
        c.char(ch, fx);
    }
}

#[test]
fn enter_sends_the_edited_line_with_its_newline() {
    let (mut c, mut fx) = (Cooked::default(), Effect::default());
    type_str(&mut c, "lx", &mut fx);
    c.backspace(&mut fx);
    type_str(&mut c, "s -l", &mut fx);
    c.enter(&mut fx);
    assert_eq!(fx.send, b"ls -l\n");
    assert!(c.line.is_empty());
}

#[test]
fn backspace_erases_a_wide_character_across_both_cells() {
    let (mut c, mut fx) = (Cooked::default(), Effect::default());
    c.char('漢', &mut fx);
    fx.echo.clear();
    c.backspace(&mut fx);
    assert_eq!(fx.echo, b"\x08 \x08\x08 \x08");
}

#[test]
fn kill_word_and_kill_line() {
    let (mut c, mut fx) = (Cooked::default(), Effect::default());
    type_str(&mut c, "git commit  ", &mut fx);
    c.kill_word(&mut fx);
    assert_eq!(c.line.iter().collect::<String>(), "git ");
    c.kill_line(&mut fx);
    assert!(c.line.is_empty());
    assert!(fx.send.is_empty(), "nothing is sent before Enter");
}

#[test]
fn a_selection_covers_whole_middle_lines() {
    let s =
        Selection { anchor: Pos { line: 5, col: 3 }, head: Pos { line: 3, col: 7 }, block: false };
    assert!(s.contains(4, 0) && s.contains(4, 500));
    assert!(s.contains(3, 7) && !s.contains(3, 6));
    assert!(s.contains(5, 3) && !s.contains(5, 4));
}

#[test]
fn a_block_selection_covers_columns() {
    let s =
        Selection { anchor: Pos { line: 1, col: 8 }, head: Pos { line: 3, col: 2 }, block: true };
    assert!(s.contains(2, 5) && !s.contains(2, 9) && !s.contains(2, 1));
}
