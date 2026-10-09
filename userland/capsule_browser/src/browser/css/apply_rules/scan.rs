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

use crate::browser::css::budget::MatchBudget;
use crate::browser::css::matching::{matches_selector, Siblings};
use crate::browser::css::rule_index::{Entry, RuleIndex};
use crate::browser::dom::Dom;

use crate::browser::css::walk::{Hit, Sheet};

/* Where one matching pass looks: a tree, and a sheet through one of its
 * indexes, with the walk's scratch list of candidate buckets. */
pub(super) struct Scan<'a, 'd> {
    pub dom: &'d Dom,
    pub sib: &'d Siblings,
    pub sheet: Sheet<'a>,
    pub index: &'a RuleIndex,
    pub buckets: &'d mut Vec<&'a [Entry]>,
}

/* The rules of one pass that element `id` matches, in cascade order. With
 * a budget, a pass that runs dry, before or between candidates, keeps no
 * hit, so the element keeps its inherited and UA style. */
pub(super) fn collect(
    scan: Scan,
    id: usize,
    hits: &mut Vec<Hit>,
    budget: Option<&mut MatchBudget>,
) {
    let Scan { dom, sib, sheet, index, buckets } = scan;
    hits.clear();
    let Some(node) = dom.nodes.get(id) else { return };
    index.candidates(node, buckets);
    let mut budget = budget;
    if budget.as_mut().is_some_and(|b| !b.take(buckets.iter().map(|s| s.len()).sum())) {
        return;
    }
    for e in buckets.iter().flat_map(|s| s.iter()) {
        if budget.as_ref().is_some_and(|b| b.spent_out()) {
            hits.clear();
            return;
        }
        let Some(rule) = sheet.rules.get(e.rule as usize) else { continue };
        let Some(sel) = rule.selectors.get(e.sel as usize) else { continue };
        if matches_selector(dom, sib, id, sel) {
            hits.push(Hit { elem: e.elem, layer: rule.layer, spec: e.spec, rule: e.rule });
        }
    }
    hits.sort_unstable_by_key(|h| (h.elem, h.layer, h.spec, h.rule));
}
