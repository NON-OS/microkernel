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

use crate::browser::css::rule::Rule;
use crate::browser::css::selector::{Pseudo, Selector, Simple};
use crate::browser::css::specificity::specificity;

use super::bucket::Buckets;
use super::key::hash_name;
use super::{Entry, RuleIndex};

/* Index the selectors of `rules` (plain ones, or with `pseudo` the
 * pseudo-element ones). A selector with a compound that can never match
 * (a Never pseudo-class at top level) is left out: testing it could only
 * fail. Classes are counted first so each selector goes under the class
 * of its key that the fewest selectors share. */
pub(super) fn build(rules: &[Rule], pseudo: bool) -> RuleIndex {
    let wanted = |s: &Selector| (s.element != 0) == pseudo && !never(s);
    let mut counts: Vec<u64> = Vec::new();
    for s in rules.iter().flat_map(|r| &r.selectors).filter(|s| wanted(s)) {
        counts.extend(s.key.classes.iter().map(|c| hash_name(c, false)));
    }
    counts.sort_unstable();
    let count = |h: u64| counts.partition_point(|&c| c <= h) - counts.partition_point(|&c| c < h);
    let mut lists: [Vec<(u64, Entry)>; 4] = Default::default();
    let mut universal = Vec::new();
    for (ri, rule) in rules.iter().enumerate() {
        for (si, s) in rule.selectors.iter().enumerate().filter(|(_, s)| wanted(s)) {
            let e =
                Entry { rule: ri as u32, sel: si as u16, elem: s.element, spec: specificity(s) };
            match bucket(&s.key, &count) {
                Some((k, h)) => lists[k].push((h, e)),
                None => universal.push(e),
            }
        }
    }
    let [ids, classes, tags, attrs] = lists.map(Buckets::from);
    RuleIndex { ids, classes, tags, attrs, universal }
}

/* The bucket a key compound goes under, as (list, name hash): its id
 * (0), else its rarest class (1), else its tag (2), else its first
 * attribute name (3); None for a universal key. */
fn bucket(key: &Simple, count: &dyn Fn(u64) -> usize) -> Option<(usize, u64)> {
    if let Some(id) = &key.id {
        return Some((0, hash_name(id, false)));
    }
    if let Some(h) = key.classes.iter().map(|c| hash_name(c, false)).min_by_key(|&h| count(h)) {
        return Some((1, h));
    }
    let tag = key.tag.as_ref().map(|t| (2, hash_name(t, true)));
    tag.or_else(|| key.attrs.first().map(|(name, _)| (3, hash_name(name, true))))
}

fn never(sel: &Selector) -> bool {
    let dead = |s: &Simple| s.pseudo.iter().any(|p| matches!(p, Pseudo::Never));
    dead(&sel.key) || sel.ancestors.iter().any(|a| dead(&a.simple))
}
