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

use crate::browser::css::selector::Pseudo;

use super::compound_part::Compound;
use super::cursor::Cur;
use super::ident::ident;

/* An optional type or universal selector with its namespace prefix: E, *,
 * ns|E, *|E or |E. No @namespace rule reaches the selector parser, so a
 * named prefix is taken as any namespace. The empty prefix asks for an
 * element in no namespace, which no element of an HTML document is.
 * Returns whether one was read; None when a prefix has no name after it. */
pub(super) fn type_selector(c: &mut Cur, out: &mut Compound) -> Option<bool> {
    let first: Option<Option<String>> = if c.eat(b'*') { Some(None) } else { ident(c).map(Some) };
    let prefixed = c.peek() == Some(b'|') && c.at(1) != Some(b'|');
    if !prefixed {
        let read = first.is_some();
        if let Some(t) = first {
            set(out, t);
        }
        return Some(read);
    }
    c.i += 1;
    let name = if c.eat(b'*') { None } else { Some(ident(c)?) };
    if first.is_none() {
        out.simple.pseudo.push(Pseudo::Never);
    }
    set(out, name);
    Some(true)
}

fn set(out: &mut Compound, t: Option<String>) {
    if let Some(t) = t {
        out.simple.tag = Some(t.to_ascii_lowercase());
        out.spec.c += 1;
    }
}
