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
use super::list_kind::{fits, Kind};
use super::skip::skip_arg;

/* A selector list after its '(', through its ')'. Returns the arguments
 * that can match and the largest specificity among them, packed. An
 * argument ending in a pseudo-element can never match an element: in a
 * forgiving or `of` list it stays valid and is left out, elsewhere it is
 * invalid. Only a forgiving list may be empty or hold empty entries. */
pub(super) fn list(c: &mut Cur, kind: Kind) -> Option<(Vec<Selector>, u32)> {
    let (mut out, mut best, mut seen) = (Vec::new(), 0u32, 0usize);
    loop {
        let mark = (c.i, c.depth, c.in_has, c.compounds);
        c.trivia();
        let got = match c.peek() {
            None | Some(b',') | Some(b')') => None,
            _ => complex(c, kind == Kind::Relative).filter(|s| fits(kind, s)),
        };
        match got {
            Some(sel) => {
                seen += 1;
                if sel.element == 0 {
                    best = best.max(sel.spec);
                    out.push(sel);
                }
            }
            None if kind == Kind::Forgiving => {
                (c.i, c.depth, c.in_has, c.compounds) = mark;
                skip_arg(c, false);
            }
            None => return None,
        }
        if !c.eat(b',') {
            break;
        }
    }
    (c.close() && (seen > 0 || kind == Kind::Forgiving)).then_some((out, best))
}
