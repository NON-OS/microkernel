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

use crate::browser::dom::node::Node;

use super::key::hash_name;
use super::{Entry, RuleIndex};

impl RuleIndex {
    /* The buckets `node` draws candidates from, into `out`. */
    pub fn candidates<'a>(&'a self, node: &Node, out: &mut Vec<&'a [Entry]>) {
        out.clear();
        let mut take = |s: &'a [Entry]| {
            if !s.is_empty() {
                out.push(s)
            }
        };
        if let Some(id) = node.attr("id").filter(|_| !self.ids.is_empty()) {
            take(self.ids.get(hash_name(id, false)));
        }
        if let Some(cls) = node.attr("class").filter(|_| !self.classes.is_empty()) {
            for (i, c) in cls.split_whitespace().enumerate() {
                if !cls.split_whitespace().take(i).any(|p| p == c) {
                    take(self.classes.get(hash_name(c, false)));
                }
            }
        }
        take(self.tags.get(hash_name(&node.tag, true)));
        if !self.attrs.is_empty() {
            for (name, _) in &node.attrs {
                take(self.attrs.get(hash_name(name, true)));
            }
        }
        take(&self.universal);
    }
}
