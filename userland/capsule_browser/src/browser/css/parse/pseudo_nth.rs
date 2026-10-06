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

use crate::browser::css::selector::{is_space, Pseudo};

use super::cursor::Cur;
use super::list::list;
use super::list_kind::Kind;
use super::nth::anb;
use super::spec::Spec;

/* The An+B family, the cursor just past '('. The two child forms take an
 * optional `of S` selector list, which adds its most specific argument to
 * the one pseudo-class they count as; the of-type forms do not. None for
 * any other name. */
pub(super) fn nth_pseudo(c: &mut Cur, name: &str, spec: &mut Spec) -> Option<Pseudo> {
    let (child, last) = match name {
        "nth-child" => (true, false),
        "nth-last-child" => (true, true),
        "nth-of-type" => (false, false),
        "nth-last-of-type" => (false, true),
        _ => return None,
    };
    c.trivia();
    let (a, b) = anb(c)?;
    let ws = c.trivia();
    spec.b += 1;
    if c.close() {
        return Some(match (child, last) {
            (true, false) => Pseudo::NthChild(a, b),
            (true, true) => Pseudo::NthLastChild(a, b),
            (false, false) => Pseudo::NthOfType(a, b),
            (false, true) => Pseudo::NthLastOfType(a, b),
        });
    }
    /* "of" is matched case-sensitively and must be set off by whitespace,
     * as Chromium reads it. */
    if !(child && ws && c.s[c.i..].starts_with("of") && c.at(2).is_some_and(is_space)) {
        return None;
    }
    c.i += 2;
    let (sels, s) = list(c, Kind::Of)?;
    *spec = spec.add(Spec::unpack(s));
    Some(if last { Pseudo::NthLastChildOf(a, b, sels) } else { Pseudo::NthChildOf(a, b, sels) })
}
