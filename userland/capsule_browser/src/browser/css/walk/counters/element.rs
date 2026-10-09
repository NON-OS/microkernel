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

use alloc::string::String;

use crate::browser::css::rule_index::key::hash_name;
use crate::browser::dom::node::Node;

use super::ops::pairs;
use super::Counters;

impl Counters {
    /* Element `id` under `parent`: its reset, increment and set, in that
     * order, with lists resetting and items stepping 'list-item'. */
    pub fn element(&mut self, node: &Node, decls: &[Option<String>; 3], parent: usize) {
        let li = hash_name("list-item", true);
        if matches!(node.tag.as_str(), "ol" | "ul" | "menu") {
            let start = node.attr("start").and_then(|s| s.trim().parse::<i32>().ok()).unwrap_or(1);
            self.reset(li, start.saturating_sub(1), parent);
        }
        for (name, n) in pairs(decls[0].as_deref(), 0) {
            self.reset(name, n, parent);
        }
        if node.tag == "li" {
            match node.attr("value").and_then(|v| v.trim().parse::<i32>().ok()) {
                Some(v) => self.set(li, v, parent),
                None => self.step(li, 1, parent),
            }
        }
        for (name, n) in pairs(decls[1].as_deref(), 1) {
            self.step(name, n, parent);
        }
        for (name, n) in pairs(decls[2].as_deref(), 0) {
            self.set(name, n, parent);
        }
    }
}
