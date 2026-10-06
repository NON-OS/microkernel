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
use crate::browser::dom::Dom;

use super::complex::{complex, Res};
use super::cx::Cx;
use super::sibling::Siblings;
use super::steps::charge;

/* Whether element `id` matches `sel` as the cascade asks it: :scope is the
 * document element. The work is bounded per call (CALL_STEPS) and counted
 * toward the process-wide step total the cascade budget reads, but a call
 * never fails because of that total, so UA matching always runs. */
pub fn matches_selector(dom: &Dom, sib: &Siblings, id: usize, sel: &Selector) -> bool {
    let (hit, spent) = matches_scoped(dom, sib, 0, id, sel);
    charge(spent);
    hit
}

/* The same with an explicit :scope element (0 for the document element),
 * returning the steps spent. A call that runs out of steps is no match,
 * whatever a negation would have concluded from the tests it cut short. */
pub fn matches_scoped(
    dom: &Dom,
    sib: &Siblings,
    scope: usize,
    id: usize,
    sel: &Selector,
) -> (bool, u32) {
    let cx = Cx::new(dom, sib, scope);
    let hit = test(&cx, id, sel) && !cx.exhausted();
    (hit, cx.spent())
}

/* One selector at one element inside a match already under way: first the
 * ancestor filter, which rejects in O(1) a selector that needs a tag, id
 * or class no ancestor has, then the full right-to-left match. */
pub(super) fn test(cx: &Cx, id: usize, sel: &Selector) -> bool {
    if cx.sib.tab().is_some_and(|t| !t.may_match(id, &sel.anc_bits)) {
        return false;
    }
    complex(cx, id, sel, 0) == Res::Matched
}
