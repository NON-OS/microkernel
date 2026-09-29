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

use super::walk::{Hit, Sheet, Walker};

impl<'a> Walker<'a> {
    /* The UA rules element `id` matches, in cascade order. The UA sheet
     * is never budgeted: base layout survives a hostile author sheet. */
    pub fn match_ua(&mut self, id: usize) {
        let Walker { dom, sib, ua, buckets, ua_hits, .. } = self;
        collect(Scan { dom, sib, sheet: *ua, index: ua.index, buckets }, id, ua_hits, None);
    }

    /* The author rules element `id` (or, with `pseudo`, one of its
     * pseudo-elements) matches, in cascade order. Each candidate test is
     * charged to the budget; once it is dry the element keeps its
     * inherited and UA style. */
    pub fn match_author(&mut self, id: usize, pseudo: bool) {
        let Walker { dom, sib, author, pseudo: pidx, buckets, author_hits, budget, .. } = self;
        let index = if pseudo { *pidx } else { author.index };
        let scan = Scan { dom, sib, sheet: *author, index, buckets };
        collect(scan, id, author_hits, Some(budget));
    }
}

/* Where one matching pass looks: a tree, and a sheet through one of its
 * indexes, with the walk's scratch list of candidate buckets. */
struct Scan<'a, 'd> {
    dom: &'d Dom,
    sib: &'d Siblings,
    sheet: Sheet<'a>,
    index: &'a RuleIndex,
    buckets: &'d mut Vec<&'a [Entry]>,
}

fn collect(scan: Scan, id: usize, hits: &mut Vec<Hit>, budget: Option<&mut MatchBudget>) {
    let Scan { dom, sib, sheet, index, buckets } = scan;
    hits.clear();
    let Some(node) = dom.nodes.get(id) else { return };
    index.candidates(node, buckets);
    if let Some(b) = budget {
        if !b.take(buckets.iter().map(|s| s.len()).sum()) {
            return;
        }
    }
    for e in buckets.iter().flat_map(|s| s.iter()) {
        let Some(rule) = sheet.rules.get(e.rule as usize) else { continue };
        let Some(sel) = rule.selectors.get(e.sel as usize) else { continue };
        if matches_selector(dom, sib, id, sel) {
            hits.push(Hit { elem: e.elem, layer: rule.layer, spec: e.spec, rule: e.rule });
        }
    }
    hits.sort_unstable_by_key(|h| (h.elem, h.layer, h.spec, h.rule));
}
