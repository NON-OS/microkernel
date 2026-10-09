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

use crate::browser::css::selector::{Comb, Selector};

use super::cx::Cx;
use super::simple::compound;

/* How a failed match tells the step to its right where a retry could still
 * succeed: from the next sibling candidate, only from the closest
 * descendant combinator further right, or nowhere at all. */
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Res {
    Matched,
    RetryLaterSibling,
    RetryDescendant,
    Nowhere,
}

/* Compound k of `sel` (0 the key, k the k-th step leftwards) at element e,
 * then everything left of it. Right to left with backtracking: a
 * descendant step tries each ancestor in turn, a subsequent-sibling step
 * each earlier sibling, and a child or next-sibling step its one
 * candidate. The failure kinds (as Servo and Blink use them) stop a retry
 * that cannot help, so a failed step is never re-tried from higher up and
 * the work stays polynomial: at most the tree depth times the compounds for
 * descendant chains. Every compound test costs one step of the budget. */
pub(super) fn complex(cx: &Cx, e: usize, sel: &Selector, k: usize) -> Res {
    let simple = if k == 0 { &sel.key } else { &sel.ancestors[k - 1].simple };
    if !cx.tick() {
        return Res::Nowhere;
    }
    if !compound(cx, e, simple) {
        return Res::RetryLaterSibling;
    }
    let Some(step) = sel.ancestors.get(k) else {
        return Res::Matched;
    };
    let sibling = step.comb.is_sibling();
    let mut next = if sibling { cx.prev_el(e) } else { cx.parent_el(e) };
    while let Some(n) = next {
        let r = complex(cx, n, sel, k + 1);
        match (r, step.comb) {
            (Res::Matched | Res::Nowhere, _) | (_, Comb::NextSibling) => return r,
            (_, Comb::Child) => return Res::RetryDescendant,
            (Res::RetryDescendant, Comb::SubsequentSibling) => return r,
            _ => {}
        }
        next = if sibling { cx.prev_el(n) } else { cx.parent_el(n) };
    }
    if sibling {
        Res::RetryDescendant
    } else {
        Res::Nowhere
    }
}
