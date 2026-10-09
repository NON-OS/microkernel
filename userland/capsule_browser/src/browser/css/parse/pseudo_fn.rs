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

use crate::browser::css::selector::Pseudo;

use super::cursor::Cur;
use super::list::list;
use super::list_kind::Kind;
use super::pseudo_arg::argument;
use super::pseudo_nth::nth_pseudo;
use super::spec::Spec;

/* Functional pseudo-classes nested deeper than this make the selector
 * invalid. Parsing and matching both recurse once per level, so this is
 * what bounds their stack. */
const MAX_DEPTH: u32 = 32;

/* A functional pseudo-class, the cursor just past its '('. :is(), :not()
 * and :has() count as the most specific of their arguments, :where() as
 * nothing, and the rest as one pseudo-class. */
pub(super) fn functional(c: &mut Cur, name: &str, spec: &mut Spec) -> Option<Pseudo> {
    if c.depth >= MAX_DEPTH {
        return None;
    }
    c.depth += 1;
    let mut most = |s: u32| *spec = spec.add(Spec::unpack(s));
    let p = match name {
        "is" => list(c, Kind::Forgiving).map(|(l, s)| {
            most(s);
            Pseudo::Matches(l)
        })?,
        "where" => Pseudo::Matches(list(c, Kind::Forgiving)?.0),
        "-webkit-any" => list(c, Kind::Compound).map(|(l, s)| {
            most(s);
            Pseudo::Matches(l)
        })?,
        "not" => list(c, Kind::Strict).map(|(l, s)| {
            most(s);
            Pseudo::Not(l)
        })?,
        "has" if !c.in_has => {
            c.in_has = true;
            let got = list(c, Kind::Relative);
            c.in_has = false;
            got.map(|(l, s)| {
                most(s);
                Pseudo::Has(l)
            })?
        }
        "lang" | "dir" | "state" | "host" | "host-context" => {
            spec.b += 1;
            argument(c, name)?
        }
        _ => nth_pseudo(c, name, spec)?,
    };
    c.depth -= 1;
    Some(p)
}
