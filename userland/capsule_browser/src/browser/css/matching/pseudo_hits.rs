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

use crate::browser::dom::Dom;

use super::super::budget::MatchBudget;
use super::super::rule::Rule;
use super::super::rule_index::RuleIndex;
use super::super::specificity::specificity;
use super::{matches_selector, Siblings};

/* The author rules whose ::before (which 1) or ::after (which 2) selectors
 * match `id`, as (specificity, rule index) in cascade order. Empty when none
 * match or the match budget is spent, the common case for most elements. */
pub(in crate::browser::css) fn pseudo_hits(
    dom: &Dom,
    sib: &Siblings,
    id: usize,
    author: (&[Rule], &RuleIndex),
    which: u8,
    budget: &mut MatchBudget,
) -> Vec<(u32, usize)> {
    let (rules, index) = author;
    let mut hits: Vec<(u32, usize)> = Vec::new();
    let cands = index.candidates(dom, id);
    if !budget.take(cands.len()) {
        return hits;
    }
    for i in cands {
        let Some(rule) = rules.get(i) else { continue };
        let mut best: Option<u32> = None;
        for sel in &rule.selectors {
            if sel.element == which && matches_selector(dom, sib, id, sel) {
                let s = specificity(sel);
                best = Some(best.map_or(s, |b| b.max(s)));
            }
        }
        if let Some(s) = best {
            hits.push((s, i));
        }
    }
    hits.sort();
    hits
}
