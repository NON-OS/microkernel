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

mod bucket;
mod build;
mod candidates;
pub(super) mod key;

use alloc::vec::Vec;

use bucket::Buckets;

/* One selector a node may match: its rule, its place in the rule's list,
 * its pseudo-element code and its specificity. */
#[derive(Clone, Copy)]
pub(super) struct Entry {
    pub rule: u32,
    pub sel: u16,
    pub elem: u8,
    pub spec: u32,
}

/* Selectors bucketed by one simple selector of their key (rightmost)
 * compound, the most selective one: id, else the rarest class, else the
 * tag, else an attribute name; the rest are universal. A node draws its
 * candidates from the buckets of its own id, classes, tag and attribute
 * names, a superset of its matches that matches_selector then decides. */
pub(super) struct RuleIndex {
    ids: Buckets,
    classes: Buckets,
    tags: Buckets,
    attrs: Buckets,
    universal: Vec<Entry>,
}

impl RuleIndex {
    /* Selectors styling the element itself, or with `pseudo` those
     * styling one of its pseudo-elements. */
    pub fn build(rules: &[crate::browser::css::rule::Rule], pseudo: bool) -> RuleIndex {
        build::build(rules, pseudo)
    }

    pub fn is_empty(&self) -> bool {
        self.universal.is_empty()
            && [&self.ids, &self.classes, &self.tags, &self.attrs].iter().all(|b| b.is_empty())
    }
}
