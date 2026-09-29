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

use crate::browser::css::selector::Selector;

use super::cx::Cx;
use super::selector::test;
use super::structural::nth;

/* :nth-child(An+B of S) and :nth-last-child(An+B of S): the element
 * matches S and its 1-based place among the element siblings (before it,
 * or after it for the last form) that also match S satisfies An+B. Each
 * sibling test costs a step. */
pub(super) fn nth_of(cx: &Cx, id: usize, (a, b): (i32, i32), of: &[Selector], last: bool) -> bool {
    let hit = |e: usize| of.iter().any(|s| test(cx, e, s));
    if !hit(id) {
        return false;
    }
    let Some(parent) = cx.dom.nodes.get(id).and_then(|n| cx.dom.nodes.get(n.parent)) else {
        return false;
    };
    let kids = &parent.children;
    let Some(at) = kids.iter().position(|&k| k == id) else {
        return false;
    };
    let others = if last { &kids[at + 1..] } else { &kids[..at] };
    let mut place: i32 = 1;
    for &k in others {
        if cx.element(k).is_none() {
            continue;
        }
        if !cx.tick() {
            return false;
        }
        place += hit(k) as i32;
    }
    nth(a, b, place)
}
