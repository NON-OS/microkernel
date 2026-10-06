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

use alloc::vec::Vec;

use crate::browser::css::selector::Selector;

use super::complex::complex;
use super::cursor::Cur;
use super::expand::expand;
use super::skip::skip_arg;

/* One top-level selector longer than this is refused before it is parsed,
 * which bounds the work any one of them can cause. */
const MAX_SELECTOR_BYTES: usize = 8192;
/* Selectors kept from one list. Longer lists stay valid and are read
 * whole; the selectors past this bound are not kept. */
const MAX_SELECTORS: usize = 1024;

/* A selector list, as a style rule or querySelectorAll reads it. The list
 * is not forgiving: one invalid selector (an unknown pseudo-class, a
 * dangling combinator, an empty entry, stray tokens) makes the whole list
 * invalid, and the result is empty, so the rule drops as it does in
 * Chromium. */
pub fn parse_selectors(list: &str) -> Vec<Selector> {
    let mut out: Vec<Selector> = Vec::new();
    let mut c = Cur::new(list);
    loop {
        let start = c.i;
        skip_arg(&mut c, true);
        let Some(sels) = top(&list[start..c.i]) else {
            return Vec::new();
        };
        let room = MAX_SELECTORS - out.len();
        out.extend(sels.into_iter().take(room));
        if !c.eat(b',') {
            return out;
        }
    }
}

fn top(part: &str) -> Option<Vec<Selector>> {
    if part.len() > MAX_SELECTOR_BYTES {
        return None;
    }
    let mut c = Cur::new(part);
    c.trivia();
    let sel = complex(&mut c, false)?;
    c.trivia();
    c.peek().is_none().then(|| expand(sel))
}
