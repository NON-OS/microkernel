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

use crate::browser::css::selector::Simple;

use super::class::{classes_match, id_matches};
use super::cx::Cx;
use super::pseudo::pseudo_matches;

/* One compound at one element: tag, id, classes, attributes, then the
 * pseudo-classes, cheapest first so most candidates fail before any
 * attribute is looked up. Only elements match. */
pub(super) fn compound(cx: &Cx, id: usize, s: &Simple) -> bool {
    let Some(node) = cx.element(id) else {
        return false;
    };
    if s.tag.as_ref().is_some_and(|t| node.tag != *t) {
        return false;
    }
    if s.id_key != 0 && !id_matches(cx, id, node, s) {
        return false;
    }
    if !classes_match(cx, id, node, s) {
        return false;
    }
    for (name, test) in &s.attrs {
        match node.attr(name) {
            Some(have) if test.matches(have) => {}
            _ => return false,
        }
    }
    s.pseudo.iter().all(|p| pseudo_matches(cx, id, p))
}
