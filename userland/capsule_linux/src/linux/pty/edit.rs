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

//! Canonical mode's line editing on a pseudo-terminal: erase, word erase,
//! kill, end of file and the end of a line, as n_tty does them. Pure.

use alloc::collections::VecDeque;

use super::flags::{cc, lflag, Termios, IEXTEN, VEOF, VEOL, VERASE, VKILL, VWERASE};
use super::input::Input;
use super::out::{echo, rub};

/// Byte `c` typed in canonical mode.
pub(super) fn edit(t: &Termios, input: &mut Input, to: &mut VecDeque<u8>, c: u8) {
    let is = |i: usize| cc(t, i) == Some(c);
    if is(VERASE) {
        erase(t, input, to, false);
    } else if is(VWERASE) && lflag(t) & IEXTEN != 0 {
        erase(t, input, to, true);
    } else if is(VKILL) {
        while !input.line.is_empty() {
            erase(t, input, to, false);
        }
    } else if is(VEOF) {
        /* Not echoed; on an empty line it is an end of file. */
        input.finish(None);
    } else if c == b'\n' || is(VEOL) {
        echo(t, c, to);
        input.finish(Some(c));
    } else if !input.full() {
        input.line.push(c);
        echo(t, c, to);
    }
}

/// One byte off the line, or under `word` the spaces before a word and it.
fn erase(t: &Termios, input: &mut Input, to: &mut VecDeque<u8>, word: bool) {
    while input.line.last().is_some_and(|&b| word && b == b' ') {
        input.line.pop().inspect(|&b| rub(t, b, to));
    }
    while input.line.last().is_some_and(|&b| !word || b != b' ') {
        input.line.pop().inspect(|&b| rub(t, b, to));
        if !word {
            break;
        }
    }
}
