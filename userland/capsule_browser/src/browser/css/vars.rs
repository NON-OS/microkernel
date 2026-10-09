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

//! Custom properties cascade like any inherited property: each element's
//! set is its parent's, shared untouched when it declares none, plus the
//! --declarations it wins, resolved against each other with cycles and
//! over-long expansions made invalid. var() resolves against that set.

mod at;
mod light_dark;
mod own;
mod props;
mod resolver;
mod scope;
mod subst;

pub(super) use at::At;
pub(super) use own::declare;
pub(super) use props::Props;
pub(super) use scope::VarScope;
pub(super) use subst::substitute;

/* Byte index of the ')' closing the '(' opened just before `start`,
 * tracking nested parentheses. */
fn find_close(s: &str, start: usize) -> Option<usize> {
    let mut depth = 1u32;
    for (i, &b) in s.as_bytes().iter().enumerate().skip(start) {
        match b {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

/* "name, fallback" split at the first top-level comma; parentheses in a
 * fallback keep their own commas out of the split. */
fn split_top_comma(inner: &str) -> (&str, Option<&str>) {
    let mut depth = 0u32;
    for (i, &b) in inner.as_bytes().iter().enumerate() {
        match b {
            b'(' => depth += 1,
            b')' => depth = depth.saturating_sub(1),
            b',' if depth == 0 => return (&inner[..i], inner.get(i + 1..)),
            _ => {}
        }
    }
    (inner, None)
}
