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

use alloc::string::String;

use crate::browser::css::selector::{is_space, AttrOp};

use super::cursor::Cur;
use super::ident::ident;

/* An attribute name after an optional namespace prefix (ns|, *| or |),
 * taken as any namespace. */
pub(super) fn attr_name(c: &mut Cur) -> Option<String> {
    let bar = |c: &Cur| c.peek() == Some(b'|') && c.at(1) != Some(b'=');
    if c.eat(b'*') {
        return if c.eat(b'|') { ident(c) } else { None };
    }
    if !bar(c) {
        let name = ident(c)?;
        if !bar(c) {
            return Some(name);
        }
    }
    c.i += 1;
    ident(c)
}

/* An unquoted value: an identifier, or else a run of anything but
 * whitespace, quotes, escapes, brackets and '='. CSS asks for an
 * identifier, but the engine's own proofs pin a value like [href$=.png] as
 * a working suffix test. */
pub(super) fn bare(c: &mut Cur) -> Option<String> {
    if let Some(v) = ident(c) {
        return Some(v);
    }
    let start = c.i;
    let stop = |b: u8| is_space(b) || b"[](){}<>=\"'\\".contains(&b);
    while c.peek().is_some_and(|b| !stop(b)) {
        c.i += 1;
    }
    let v = c.s.get(start..c.i).filter(|v| !v.is_empty())?;
    Some(String::from(v))
}

pub(super) fn attr_op(c: &mut Cur) -> Option<AttrOp> {
    if c.eat(b'=') {
        return Some(AttrOp::Eq);
    }
    let op = match (c.peek()?, c.at(1)?) {
        (b'~', b'=') => AttrOp::Word,
        (b'|', b'=') => AttrOp::Lang,
        (b'^', b'=') => AttrOp::Starts,
        (b'$', b'=') => AttrOp::Ends,
        (b'*', b'=') => AttrOp::Contains,
        _ => return None,
    };
    c.i += 2;
    Some(op)
}
